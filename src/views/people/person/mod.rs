use dioxus::prelude::*;

use crate::state::client::PEEPS;

mod buttons;
mod container;
mod img;
mod prompt;
mod svg_quote_mark;

use img::Image;
use prompt::Prompt;

#[component]
pub fn Person(id: String) -> Element {
    let person = PEEPS.iter().find(|p| p.id == id).unwrap();

    let mut pics = person.pictures.clone().into_iter();
    let mut prompts = person.prompts.clone().into_iter();

    rsx! {

        div { class: "relative md:columns-3 *:mb-2",
            h2 { "{person.name}" }

            Image { id: id.clone(), src: pics.next() }
            Prompt { id: id.clone(), pp: prompts.next() }

            Image { id: id.clone(), src: pics.next() }
            Prompt { id: id.clone(), pp: prompts.next() }

            Image { id: id.clone(), src: pics.next() }
            Prompt { id: id.clone(), pp: prompts.next() }

            Image { id: id.clone(), src: pics.next() }
            Prompt { id: id.clone(), pp: prompts.next() }

            Image { id: id.clone(), src: pics.next() }
            Prompt { id: id.clone(), pp: prompts.next() }

            Image { id: id.clone(), src: pics.next() }
            Prompt { id: id.clone(), pp: prompts.next() }

            buttons::DislikeButton { id }
        }

    }
}
