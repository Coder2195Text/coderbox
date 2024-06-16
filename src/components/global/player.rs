use std::time::Duration;

use dioxus::prelude::*;

use crate::components::icons::fa::FaRegCirclePause;
use crate::components::icons::fa::FaRegCirclePlay;
use crate::SINK_INSTANCE;

#[component]
pub fn Player() -> Element {
  let mut playing = use_signal(|| false);

  let sink_instance = SINK_INSTANCE.read().unwrap();
  let sink = sink_instance.as_ref().unwrap();
  if playing() {
    sink.play();
  } else {
    sink.pause();
  }

  rsx! {
    div { class: "m-2 rounded-md bg-gradient-to-br from-[#ffffff08] to-[#ffffff13] p-2 gap-2 flex",
      div { class: "w-1/4",
        "Name"
        br {}
        "Artist"
      }
      div { class: "w-1/2 rounded-full flex flex-col",
        input {
          r#type: "range",
          min: 0,
          max: 100,
          value: 0,
          class: "w-full h-4 rounded-full bg-teal-500",
          onchange: move |e| {
              let sink_instance = SINK_INSTANCE.read().unwrap();
              let sink = sink_instance.as_ref().unwrap();
              sink.try_seek(Duration::from_secs(e.value().parse().expect("not number")))
                  .expect("issue with changing");
          }
        }
        button {
          class: "mx-auto",
          onclick: move |_| { playing.set(!playing()) },
          {
            if playing() {
              rsx! {
                FaRegCirclePause {
                    size: "32px",
                }
              }
            } else {
              rsx! {
                FaRegCirclePlay {
                    size: "32px",
                }
              }
            }
          }
        }
      }
      div { class: "w-1/4",
        "Buttons"
        br {}
        "Volume"
      }
    }
  }
}
