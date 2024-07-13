use std::path::PathBuf;

use crate::{components::cards::playlist::PlaylistCard, structs::music_db::MusicDB};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
  let db = use_context::<Signal<MusicDB>>();

  rsx! {
    div {
      h1 { "Home" }
      div { class: "flex flex-wrap gap-3",
        PlaylistCard {
          id: 0,
          name: "My Songs".to_string(),
          image: PathBuf::from("/home/coder2195/Downloads/pack.png")
        }
        {
          db.read().get_playlists().iter().map(|(key, playlist)| if playlist.id() == 0 {
            None
          } else {
            rsx! {
              PlaylistCard {
                key: "{key}",
                id: playlist.id(),
                name: playlist.name(),
                image: playlist.image().cloned()
              }
            }.into()
          })
        }
      }
    }
  }
}
