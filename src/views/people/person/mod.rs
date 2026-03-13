use dioxus::prelude::*;

use crate::state::client::PEEPS;

mod buttons;
mod container;
mod img;
mod prompt;

use img::Image;
use prompt::Prompt;

#[component]
pub fn Person(id: String) -> Element {
    let person = PEEPS.iter().find(|p| p.id == id).unwrap();

    let mut pics = person.pictures.clone().into_iter();
    let mut prompts = person.prompts.clone().into_iter();

    rsx! {

        div { class: "flex flex-wrap gap-2",
            h2 { "{person.name}" }

            Image { src: pics.next() }
            Prompt { pp: prompts.next() }

            Image { src: pics.next() }
            Prompt { pp: prompts.next() }

            Image { src: pics.next() }
            Prompt { pp: prompts.next() }

            Image { src: pics.next() }
            Prompt { pp: prompts.next() }

            Image { src: pics.next() }
            Prompt { pp: prompts.next() }

            Image { src: pics.next() }
            Prompt { pp: prompts.next() }

            buttons::LikeButton { id: id.clone(), person_liked: person.liked }
            buttons::DislikeButton { id, person_liked: person.liked }
        }

    }
}
