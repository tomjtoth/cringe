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
    state::{
        websocket::{WsCtx, WsRequest},
        ME,
    },
    views::people::{
        listing::ListingCtx,
        profile::{container::Container, ProfileCtx, ResourceCtx},
    },
};

#[component]
pub fn Details() -> Element {
    let olcx = use_context::<Option<ListingCtx>>();
    let pcx = use_context::<ProfileCtx>();
    let mut rcx = use_context::<ResourceCtx>();
    let wscx = use_context::<WsCtx>();

    use_effect(move || rcx.await_op());

    let onsubmit = use_callback(move |_: Event<FormData>| {
        spawn(async move {
            if let Some(me) = ME.with(|me| me.draft.clone()) {
                _ = wscx.req(WsRequest::DetailsUpdate(rcx.op_id, *me)).await;
            }
        });
    });

    let values_under_ul = pcx.with(|profile| {
        let selectors: [fn(&Profile) -> bool; 5] = [
            |p| p.occupation.is_some(),
            |p| p.education.is_some(),
            |p| p.hometown.is_some(),
            |p| p.seeking.is_some(),
            |p| p.relationship_type.is_some(),
        ];

        selectors.iter().filter(|sel| sel(&profile)).count()
    });

    let class_container = format!(
        "px-2 [&>*+*]:border-t [&>*+*]:p-2 {} {} {}{}{}",
        "[&_input]:border-none! [&>div>input]:w-full",
        "[&_select]:border-none! [&>div>select]:px-0! [&>div>select]:w-full",
        "[&>div]:flex [&>div]:gap-2 [&>div]:items-center",
        if rcx.editing() {
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
        if olcx.is_none() && !rcx.editing() && values_under_ul < 2 {
            " [&>li:last-child]:mr-15"
        } else {
            ""
        }
    );

    let immutables = "cursor-not-allowed select-none text-gray-500";

    rsx! {
        Container { class: class_container, wo_button: olcx.is_some(), onsubmit,
            ul { class: class_ul,

                if let Some(age) = &pcx.read().age() {
                    li { class: if rcx.editing() { immutables }, "🎂 {age}" }
                }

                if let Some(dist) = &pcx.read().distance() {
                    li { class: if rcx.editing() { immutables }, "{dist}" }
                }

                li { class: if editing() { immutables }, "{sig.read().gender}" }

                li { class: if rcx.editing() { immutables }, "📏 {pcx.read().height} cm" }

                location::Location {}

                has_children::HasChildren {}
                family_plans::FamilyPlans {}

                // TODO: include pets here, could be a u8/int2 in DB
                // 0 - Doesn't have
                //
                if let Some(sign) = pcx.read().zodiac_sign() {
                    li { class: if rcx.editing() { "cursor-not-allowed" }, "{sign}" }
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
