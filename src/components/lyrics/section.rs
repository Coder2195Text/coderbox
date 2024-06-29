use std::time::Duration;

use dioxus::prelude::*;

use crate::{
  structs::{lyrics::LyricLine, song::SongData},
  SINK_INSTANCE,
};

#[derive(PartialEq, Clone, Props)]
pub struct Props {
  lyrics: LyricLine,
}

#[component]
pub fn LyricSection(props: Props) -> Element {
  let Props { lyrics } = props;

  let mut song_data = use_context::<Signal<SongData>>();
  let time = song_data.read().0;

  let color = if time >= lyrics.start() && time < lyrics.start() + lyrics.duration() {
    "text-teal-500"
  } else {
    "text-white"
  };

  rsx! {
    div {
      button {
        class: format!("text-xl font-bold {color}"),
        onclick: move |_| {
            let sink_instance = SINK_INSTANCE.read().unwrap();
            let sink = sink_instance.as_ref().unwrap();
            sink.try_seek(Duration::from_secs_f64(lyrics.start())).ok();
            song_data.write().0 = lyrics.start();
        },
        {
          lyrics.content().to_string()
        }
      }
    }
  }
}
