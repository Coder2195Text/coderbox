use std::time::Duration;

use dioxus::prelude::*;

use crate::{
  structs::{lyrics::LyricLine, song::CurrentTime},
  SINK_INSTANCE,
};

#[derive(PartialEq, Clone, Props)]
pub struct Props {
  lyrics: LyricLine,
}

#[component]
pub fn LyricSection(props: Props) -> Element {
  let Props { lyrics } = props;

  let mut time = use_context::<Signal<CurrentTime>>();

  // skibi skibid skibidi toilet sigma rizzlet fanum tax
  let color = if time().0 >= lyrics.start() && time().0 < lyrics.start() + lyrics.duration() {
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
            time.write().0 = lyrics.start();
        },
        {
          lyrics.content().to_string()
        }
      }
    }
  }
}
