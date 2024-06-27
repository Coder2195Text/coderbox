use std::time::Duration;

use dioxus::prelude::*;

use crate::{structs::lyrics::LyricLine, SINK_INSTANCE};

#[derive(PartialEq, Clone, Props)]
pub struct Props {
  lyrics: LyricLine,
}

#[component]
pub fn LyricSection(props: Props) -> Element {
  let Props { lyrics } = props;

  let mut current_time = use_context::<Signal<f64>>();
  let time = current_time();

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
            current_time.set(lyrics.start());
        },
        {
          lyrics.content().to_string()
        }
      }
    }
  }
}
