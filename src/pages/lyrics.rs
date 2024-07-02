use dioxus::prelude::*;

use crate::{components::lyrics::section::LyricSection, structs::song::Song};

#[component]
pub fn Lyrics() -> Element {
  let song = use_context::<Signal<Option<Song>>>();
  let current_song = song();

  rsx! {
    div {
      h1 { "Lyrics" }
      {
        if let Some(song) = current_song {
          if let Some(lyrics) = song.lyrics_content() {
            rsx! {
              div {
                {
                  lyrics.iter().map(|line| {
                    rsx! {
                      LyricSection { lyrics: line.clone() }
                    }
                  })
                }
              }
            }
          } else {
            rsx!{div { "No lyrics found" }}
          }
        } else {
          rsx!{div { "No song selected" }}
        }
      }
    }
  }
}
