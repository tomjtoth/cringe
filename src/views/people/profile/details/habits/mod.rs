use dioxus::prelude::*;

mod generic;

#[component]
pub(super) fn Habits() -> Element {
    rsx! {
        generic::Habit {
            emoji: "🍷",
            selector: |h| h.drinking.as_ref(),
            onchange: |h, f| h.drinking = f,
            option { value: "", "Drinking?" }
        }

        generic::Habit {
            emoji: "🚬",
            selector: |h| h.smoking.as_ref(),
            onchange: |h, f| h.smoking = f,
            option { value: "", "Smoking?" }
        }

        generic::Habit {
            emoji: "🌿🚬",
            selector: |h| h.marijuana.as_ref(),
            onchange: |h, f| h.marijuana = f,
            option { value: "", "Marijuana?" }
        }

        generic::Habit {
            emoji: "💊💉",
            selector: |h| h.drugs.as_ref(),
            onchange: |h, f| h.drugs = f,
            option { value: "", "Drugs?" }
        }

    }
}
