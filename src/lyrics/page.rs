use dioxus::prelude::*;

use super::section::LyricSection;
use crate::structs::{lyrics::Lyrics as LyricsType, song::Song};

#[component]
pub fn Lyrics() -> Element {
  let song = use_context::<Signal<Option<Song>>>();
  let current_song = song();

  let mut lyrics =
    use_signal::<Result<LyricsType, String>>(|| Err("Loading Lyrics...".to_string()));

  use_coroutine(|_: UnboundedReceiver<()>| async move {
    if let Some(song) = current_song {
      let lyrics_content = song.lyrics_content().await;
      lyrics.set(lyrics_content);
    } else {
      lyrics.set(Err("No song selected".to_string()));
    }
  });

  // hacky solution :skull:
  // confused rn
  rsx! {
    div {
      h1 { "Lyrics" }
      {
        match lyrics() {
          Ok(lyrics) => {
            rsx!( div {{
                lyrics.iter().map(|line| {
                  rsx! (LyricSection { lyrics: line.clone() })
                })
              }})
          }

          Err(err) => {
            rsx!(div { {err} })
          }
        }
      }
    }
  }
}
