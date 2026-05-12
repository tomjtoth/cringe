mod age;
mod children;
mod distance;
mod gender;
mod generics;
mod height;
mod image_count;

use dioxus::prelude::*;

use crate::{
    components::modal::{TrModal, MODALS},
    models::{Decision, Filters as MFilters},
    state::{filters::update_filters, ME},
    views::listing::LCX,
};

type FiltersCtx = Signal<MFilters>;

#[component]
pub fn Filters() -> Element {
    let mut wants = use_signal(|| LCX.read().flatten());

    let fcx = use_signal(|| {
        ME.with(|me| {
            me.profile
                .as_ref()
                .and_then(|p| p.filters.clone())
                .unwrap_or_default()
        })
    });

    use_context_provider(|| fcx);

    // Macros inlined below; removed macro_rules definitions.

    rsx! {
        form {
            class: "flex flex-col items-center gap-2",
            class: "[&>div]:p-2 [&>div]:flex [&>div]:items-center [&>div]:gap-2",
            class: "[&>div_input[type=number]]:w-15",

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
