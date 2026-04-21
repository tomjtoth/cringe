mod dropdowns;
mod education;
mod family_plans;
mod habits;
mod has_children;
mod hometown;
mod location;
mod occupation;

use dioxus::prelude::*;

use crate::{
    models::Profile,
    state::websocket::WsCtx,
    views::people::{
        listing::ListingCtx,
        profile::{container::Container, ProfileCtx, ResourceCtx},
    },
};

#[derive(Clone)]
struct DetailsCtx {
    ro: ReadSignal<Profile>,
    rw: Signal<Profile>,
}

#[component]
pub fn Details() -> Element {
    let olcx = use_context::<Option<ListingCtx>>();
    let person = use_context::<ProfileCtx>().profile;
    let mut rcx = use_context::<ResourceCtx>();
    let wscx = use_context::<WsCtx>();
    let mut editing = use_signal(|| false);
    let sig = use_signal(|| person());

    use_context_provider(|| DetailsCtx {
        rw: sig,
        ro: person,
    });

    use_effect(move || editing.set(olcx.is_none() && rcx.editing()));

    use_effect(move || rcx.await_op());

    let onsubmit = use_callback(move |_: Event<FormData>| {
        spawn(async move {
            // sending without images or prompts to save bandwidth
            let mut me = sig();
            me.prompts.truncate(0);
            me.images.truncate(0);

            _ = wscx
                .req(crate::state::websocket::WsRequest::DetailsUpdate(
                    rcx.op_id, me,
                ))
                .await;
        });
    });

    let values_under_ul = [
        sig.read().occupation.is_some(),
        sig.read().education.is_some(),
        sig.read().hometown.is_some(),
        sig.read().seeking.is_some(),
        sig.read().relationship_type.is_some(),
    ]
    .into_iter()
    .filter(|&x| x)
    .count();

    let class_container = format!(
        "px-2 [&>*+*]:border-t [&>*+*]:p-2 {} {} {}{}{}",
        "[&_input]:border-none! [&>div>input]:w-full",
        "[&_select]:border-none! [&>div>select]:px-0! [&>div>select]:w-full",
        "[&>div]:flex [&>div]:gap-2 [&>div]:items-center",
        if editing() {
            " [&>div]:nth-last-2:mb-20"
        } else {
            ""
        },
        if olcx.is_none() && values_under_ul == 0 {
            // the edit button has bottom-5 and its top border is not even visible,
            // overriding from here to complicate things less (?)
            " [&>button]:nth-2:bottom-2!"
        } else {
            ""
        }
    );

    let class_ul = format!(
        "p-2 flex overflow-x-scroll text-nowrap {} {}{}",
        "[&>*+*]:ml-2 [&>*+*]:border-l *:p-2",
        "[&>li]:flex [&>li]:gap-2 [&>li]:items-center",
        if olcx.is_none() && !rcx.editing() && values_under_ul == 0 {
            " [&>li:last-child]:mr-15"
        } else {
            ""
        }
    );

    let immutables = "cursor-not-allowed select-none text-gray-500";

    rsx! {
        Container { class: class_container, wo_button: olcx.is_some(), onsubmit,
            ul { class: class_ul,

                if let Some(age) = &sig.read().age() {
                    li { class: if editing() { immutables }, "🎂 {age}" }
                }

                if let Some(dist) = &sig.read().distance() {
                    li { class: if editing() { immutables }, "{dist}" }
                }

                li { class: if editing() { immutables }, "{sig.read().gender}" }

                li { class: if editing() { immutables }, "📏 {sig.read().height} cm" }

                location::Location {}

                has_children::HasChildren {}
                family_plans::FamilyPlans {}

                // TODO: include pets here
                //
                if let Some(sign) = sig.read().zodiac_sign() {
                    li { class: if editing() { "cursor-not-allowed" }, "{sign}" }
                }

                habits::Habits {}

            }

            occupation::Occupation {}

            education::Education {}

            hometown::Hometown {}

            dropdowns::Seeking {}

            dropdowns::RelationshipType {}
        }
    }
}
