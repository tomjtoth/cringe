mod age;
mod children;
mod distance;
mod gender;
mod generics;
mod height;
mod image_count;
mod listing_selector;
mod mode_selector;

use dioxus::prelude::*;

use crate::{
    components::modal::{TrModal, MODALS},
    models::Filters as MFilters,
    state::{filters::update_filters, ME},
    views::listing::LCX,
};

type FiltersCtx = Signal<MFilters>;

#[component]
pub fn Filters() -> Element {
    let wants = use_signal(|| LCX.read().flatten());

    let mut fcx = use_signal(|| {
        ME.with(|me| {
            me.profile
                .as_ref()
                .and_then(|p| p.filters.clone())
                .unwrap_or_default()
        })
    });

    use_context_provider(|| fcx);

    rsx! {
        form {
            class: "h-full overflow-y-scroll flex flex-col items-center gap-2",
            class: "[&_div]:p-2 [&_div]:flex [&_div]:items-center [&_div]:gap-2",
            class: "[&_input[type=number]]:no-spinner [&_input[type=number]]:w-15",
            class: "[&_input[type=checkbox]]:hidden",
            class: "[&_label]:cursor-pointer [&_hr]:w-full",

            onsubmit: move |evt| async move {
                evt.prevent_default();

                // send one to API, then consume the other locally
                let to_api = fcx();
                let local = to_api.clone();

                if let Ok(true) = update_filters(to_api).await {
                    ME.with_mut(|me| {
                        if let Some(p) = me.profile.as_mut() {
                            p.filters = Some(local);
                        }
                    });

                    LCX.with_mut(|lcx| *lcx = Some(wants()));
                    MODALS.pop();
                }

            },

            div { class: "w-full sticky top-0 bg-background justify-between",
                span { class: "text-xl", "Filter profiles" }
                mode_selector::ModeSelector {}
                button {
                    onclick: move |evt| {
                        evt.prevent_default();
                        fcx.set(MFilters::default());
                    },

                    "🔁 Reset"

                }
                button { "💾 Save" }
            }

            div { class: "justify-between",
                gender::Gender {}
                listing_selector::ListingSelector { wants }
            }
            hr {}
            generics::GenderIdentity {}
            hr {}
            age::Age {}
            height::Height {}
            distance::Distance {}
            image_count::ImageCount {}

            hr {}

            generics::Seeking {}
            hr {}
            generics::RelationshipType {}
            hr {}
            generics::ZodiacSign {}
            hr {}
            children::Children {}
            generics::FamilyPlans {}
            hr {}
            generics::Habits {}
        }
    }
}
