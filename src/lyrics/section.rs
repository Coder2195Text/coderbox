use dioxus::prelude::*;

use crate::{
  structs::{
    lyrics::LyricLine,
    song::{CurrentTime, Song},
  },
  utils::player::set_play,
};

#[derive(PartialEq, Clone, Props)]
pub struct Props {
  lyrics: LyricLine,
}

#[component]
pub fn LyricSection(props: Props) -> Element {
  let Props { lyrics } = props;

  let time = use_context::<Signal<CurrentTime>>();

  // skibi skibid skibidi toilet sigma rizzlet fanum tax
  let color = if time().0 >= lyrics.start() && time().0 < lyrics.start() + lyrics.duration() {
    "text-teal-500"
  } else {
    "text-white"
  };

  let current_song = use_context::<Signal<Option<Song>>>();
  let song = current_song();

  rsx! {
    div { id: "l-{lyrics.start()}",
      button {
        class: format!("text-xl font-bold text-left {color}"),
        onclick: move |_| {
            if let Some(song) = &song {
                set_play(
                    song.clone(),
                    current_song,
                    lyrics.start().into(),
                    time.into(),
                    None,
                    None,
                );
            }
        },
        {
          lyrics.content().to_string()
        }
      }
    }
  }
}
