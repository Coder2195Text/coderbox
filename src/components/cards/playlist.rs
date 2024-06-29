use std::path::PathBuf;

use dioxus::prelude::*;

use crate::structs::playlist::Playlist;

#[derive(Clone, Debug, PartialEq, Props)]
pub struct PlaylistCardProps {
  pub name: String,
  pub image: Option<PathBuf>,
  pub id: String,
}

#[component]
pub fn PlaylistCard(props: PlaylistCardProps) -> Element {
  let PlaylistCardProps { name, image, id } = props;
  let image = image
    .map(|image| image.to_string_lossy().to_string())
    .unwrap_or("".into());

  let current = use_context::<Signal<Playlist>>();

  let is_current = current.read().id() == id;

  rsx! {
    div { class: "bg-gray-500 bg-opacity-30 rounded-md p-2",
      div { class: "w-40 h-40",
        img {
          src: image,
          class: "object-cover rounded-md",
          draggable: false
        }
      }
      div { class: "flex flex-row",
        h6 { class: "flex-grow", {name} }
        {

        }
      }
    }
  }
}
