use dioxus::{fullstack::MultipartFormData, prelude::*};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::types::Json;

use crate::{
    models::image::Image,
    state::client::{AUTH_CTE, ME},
    views::people::person::{container::Container, despair::Despair, ResourceCtx},
};

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Default, Debug)]
struct Response {
    authorized: bool,
    sorted: i32,
    updated: i32,
    deleted: i32,
    inserted_id: Option<i32>,
    inserted_url: Option<String>,
}

#[put("/api/me/images")]
async fn upload_image(mut form: MultipartFormData) -> Result<Response> {
    use crate::state::server::{converter_tx, get_ctx};

    let mut res = Response::default();

    if let (Some(sess_id), pool) = get_ctx().await {
        let mut id = None;
        let mut bytes = None;
        let mut prompt = None;
        let mut position = None;
        let mut sorter_ids = vec![];
        let mut sorter_pos = vec![];

        while let Ok(Some(field)) = form.next_field().await {
            if let Some(name) = field.name() {
                match name {
                    "id" => {
                        if let Ok(text) = field.text().await {
                            if let Ok(parsed) = text.parse::<i32>() {
                                id = Some(parsed);
                            }
                        }
                    }

                    "image" => {
                        if let Ok(b) = field.bytes().await {
                            bytes = Some(b.to_vec())
                        }
                    }

                    "position" => {
                        if let Ok(text) = field.text().await {
                            if let Ok(p) = text.parse::<i16>() {
                                // this comes from the HTML input
                                // of which value I cannot affect
                                position = Some(p - 1)
                            }
                        }
                    }

                    "prompt" => {
                        if let Ok(text) = field.text().await {
                            if text != "" {
                                prompt = Some(text)
                            }
                        }
                    }

                    "sorter" => {
                        if let Ok(sorter) = serde_json::from_str::<
                            crate::views::people::person::Sorter,
                        >(&field.text().await?)
                        {
                            (sorter_ids, sorter_pos) = sorter.into_iter().unzip();
                        }
                    }

                    _ => (),
                }
            }
        }

        Json(res) = sqlx::query_scalar::<_, Json<Response>>(&format!(
            r"
            WITH {AUTH_CTE},

            me AS (
                SELECT u.id FROM users u
                INNER JOIN auth a ON a.email = u.email
            ),

            arg_sorter AS (
                SELECT * FROM unnest($2::int[], $3::smallint[]) 
                AS _x(id, position)
            ),

            queue AS (
                SELECT count(*) AS idx FROM user_images
                WHERE user_id > 0 AND bytes IS NULL
            ),

            arg_image AS (
                SELECT
                    $4::int AS id,
                    $5::text AS prompt,
                    $6::int2 AS position,
                    placeholder_url(idx) AS url
                FROM queue
            ),

            deleted AS (
                DELETE FROM user_images AS ui
                USING arg_image i
                CROSS JOIN me
                WHERE ui.user_id = me.id
                AND i.id = ui.id AND i.position IS NULL
                RETURNING ui.id
            ),

            sorted AS (
                UPDATE user_images ui
                SET
                    position = s.position
                FROM arg_sorter s
                CROSS JOIN me
                WHERE ui.user_id = me.id AND ui.id = s.id
                RETURNING ui.id
            ),

            updated AS (
                UPDATE user_images AS ui
                SET
                    prompt = i.prompt,
                    position = i.position
                FROM arg_image i
                CROSS JOIN me
                WHERE ui.user_id = me.id
                AND ui.id = i.id AND i.position IS NOT NULL
                RETURNING ui.id
            ),

            inserted AS (
                INSERT INTO user_images AS ui
                    (user_id, url, prompt, position)
                SELECT
                    me.id, url, prompt, position
                FROM arg_image i
                CROSS JOIN me
                WHERE i.id IS NULL
                RETURNING ui.id, ui.url
            )

            SELECT jsonb_build_object(
                'authorized', (SELECT count(*) > 0 FROM me),
                'deleted', (SELECT count(*) FROM deleted),
                'sorted', (SELECT count(*) FROM sorted),
                'updated', (SELECT count(*) FROM updated),
                'inserted_id', (SELECT id FROM inserted),
                'inserted_url', (SELECT url FROM inserted)
            )
            "
        ))
        .bind(&sess_id)
        .bind(&sorter_ids)
        .bind(&sorter_pos)
        .bind(&id)
        .bind(&prompt)
        .bind(&position)
        .fetch_one(&pool)
        .await?;

        if let Some(id) = res.inserted_id {
            if let Some(bytes) = bytes {
                info!("Sending job #{id}");
                converter_tx().await?.send((id, bytes)).await?;
            }
        }
    }

    Ok(res)
}

#[component]
pub fn ImageEditor(src: Option<Image>) -> Element {
    let rcx = use_context::<ResourceCtx>();

    let max = ME
        .read()
        .profile
        .as_ref()
        .map(|p| p.images().len() as i16)
        .unwrap_or(0)
        + {
            // only a new image can be added as additional
            if src.is_none() {
                1
            } else {
                0
            }
        };

    let mut sig = use_signal(|| {
        (
            src.unwrap_or(Image::Uploaded {
                id: None,
                bytes: None,
                url: None,
                prompt: None,
                position: Some(max - 1),
            }),
            ME.read()
                .profile
                .as_ref()
                .map(|p| p.images().clone())
                .unwrap_or_default(),
            vec![],
        )
    });

    let sorter = use_callback(move |pos: Option<i16>| {
        sig.with_mut(|(image, draft, positions)| {
            image.set_pos(pos);

            // rm and reinsert at proper pos in the vec
            // achieve DELETE by not reinserting
            draft.retain(|img| img.id() != image.id());
            if let Some(pos) = pos {
                draft.insert(pos as usize, image.clone());
            };

            positions.truncate(0);
            for (idx, img) in draft.iter_mut().enumerate() {
                let idx = Some(idx as i16);

                if *img.pos() != idx {
                    img.set_pos(idx);
                    positions.push((img.id(), idx));
                }
            }
        })
    });

    let onsubmit = use_callback({
        let rcx = rcx.clone();

        move |evt: Event<FormData>| {
            spawn({
                let mut rcx = rcx.clone();

                async move {
                    rcx.next_state();

                    if let Ok(Response {
                        authorized: true,
                        inserted_id,
                        inserted_url,
                        ..
                    }) = upload_image(evt.into()).await.map(|res| {
                        info!("PUT /api/me/prompts returned: {:?}", &res);
                        res
                    }) {
                        ME.with_mut(|me| {
                            me.profile.as_mut().map(|p| {
                                sig.with_mut(|s| {
                                    // updated inserted image id + url
                                    if let Some(id) = inserted_id {
                                        if let Some(url) = inserted_url {
                                            if let Some(pos) = s.0.pos() {
                                                s.1.get_mut(*pos as usize).map(|i| {
                                                    i.set_id(Some(id));
                                                    i.set_url(Some(url));
                                                });
                                            }
                                        }
                                    }

                                    // finalize draft
                                    p.images.truncate(0);
                                    p.images.append(&mut s.1);
                                });
                            });
                        });
                    }

                    rcx.next_state();
                }
            });
        }
    });

    let to_be_deleted = sig.with(|(img, ..)| img.id().is_some() && img.pos().is_none());

    let new_but_empty = false;

    let button_class = "z-2 absolute bottom-5 right-5 border-2! bg-background select-none";

    let class = "pt-10 pb-20 px-2 grid grid-cols-[1fr_auto] gap-2 [&>input]:text-xl";
    rsx! {
        Container { class, onsubmit,

            if new_but_empty {
                button { class: button_class, "Select an image 1st! ☝️" }
            }

            if to_be_deleted {
                Despair {}
                button { class: button_class, "That's how you delete! 😱" }
            }

            input { name: "id", hidden: true, value: sig.read().0.id() }

            input {
                name: "sorter",
                hidden: true,
                value: serde_json::to_string(&sig.read().2)?,
            }

            input {
                name: "prompt",
                placeholder: "Prompt if any",
                class: "w-full min-w-30",
                value: sig.read().0.prompt(),
                oninput: move |evt| {
                    sig
                        .with_mut(|(img, draft, ..)| {
                        img.set_prompt(evt.value());

                        if let Some(pos) = img.pos() {
                            draft
                                .get_mut(*pos as usize)
                                .map(|dr_img| {
                                    *dr_img = img.clone();
                                });
                        }
                    })
                },
            }

            input {
                name: "position",
                placeholder: "#",
                class: "max-w-20",
                r#type: "number",
                min: 1,
                max,
                value: sig.read().0.pos().map(|p| p + 1),
                oninput: move |evt| sorter(
                    evt
                        .value()
                        .parse::<i16>()
                        .ok()
                        .filter(|&pos| 1 <= pos && pos <= max)
                        .map(|pos| pos - 1),
                ),
            }

            label { class: "col-span-2 cursor-pointer",

                if sig.with(|(img, ..)| { img.has_url() || img.has_bytes() }) {
                    img { class: "object-cover w-full", src: sig.read().0.src() }
                } else {
                    p { class: "p-5 text-9xl text-center select-none", "🖼️" }
                }

                input {
                    name: "image",
                    hidden: true,
                    r#type: "file",
                    accept: ".png,.jpg,.jpeg,.bmp",
                    onchange: move |evt| {
                        spawn(async move {
                            if let Some(file) = evt.files().get(0) {
                                if let Ok(bytes) = file.read_bytes().await {
                                    sig.write().3.set_bytes(bytes.to_vec());
                                }
                            }
                        });
                    },
                }
            }
        }
    }
}
