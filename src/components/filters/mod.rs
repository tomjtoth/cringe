mod age;
mod children;
mod distance;
mod family_plans;
mod gender;
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

            h3 { "Filter profiles" }

            for (val , (ico , msg) , shadow , checked) in [
                (Some(Decision::Skip), "🚫 skipped", "text-shadow-red-500"),
                (None, "😬 cringe", "text-shadow-yellow-500"),
                (Some(Decision::Like), "✅ liked", "text-shadow-green-500"),
            ]
                .map(|(a, b, c)| (a, b.split_once(" ").unwrap(), c, a == wants()))
            {
                label {
                    class: "cursor-pointer",
                    class: if !checked { "text-gray-500" },
                    input {
                        tabindex: -1,
                        r#type: "radio",
                        required: true,
                        name: "wants",
                        class: "border-none! appearance-none checked:text-sha",
                        checked,
                        onclick: move |_| wants.set(val),
                    }
                    span { class: if checked { "text-shadow-[0_0_1px,0_0_2px,0_0_3px,0_0_4px,0_0_5px] {shadow}" },
                        "{ico}"
                    }
                    " {msg}"
                }
            }

            gender::Gender {}
            age::Age {}
            height::Height {}
            distance::Distance {}
            children::Children {}
            family_plans::FamilyPlans {}
            image_count::ImageCount {}

            button { "Save" }
        }
    }
}
