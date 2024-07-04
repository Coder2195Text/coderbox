use std::path::PathBuf;

use crate::components::cards::playlist::PlaylistCard;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
  rsx! {
    div {
      h1 { "Home" }
      PlaylistCard {
        id: 0,
        name: "My Songs".to_string(),
        image: PathBuf::from("/home/coder2195/Downloads/pack.png")
      }
    }
  }
}
