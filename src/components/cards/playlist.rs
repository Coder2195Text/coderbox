use std::{path::PathBuf, time::Duration};

use dioxus::prelude::*;

use crate::structs::{playlist::Playlist, song::Playing};
use rand::{distributions::Uniform, Rng};

#[derive(Clone, Debug, PartialEq, Props)]
pub struct PlaylistCardProps {
  pub name: String,
  pub image: Option<PathBuf>,
  pub id: i32,
}

#[component]
pub fn PlaylistCard(props: PlaylistCardProps) -> Element {
  let PlaylistCardProps { name, image, id } = props;
  let image = image
    .map(|image| image.to_string_lossy().to_string())
    .unwrap_or("".into());

  let mut hovered = use_signal(|| false);

  let current = use_context::<Signal<Playlist>>();

  let playing = use_context::<Signal<Playing>>();

  let mut sound_waves = use_signal(|| vec![2.0; 6]);

  let moved_id = id.clone();

  let _task: Coroutine<()> = use_coroutine(|_| async move {
    loop {
      tokio::time::sleep(Duration::from_millis(500)).await;
      if playing().0 && current().id() == moved_id {
        let array: Vec<f64> = rand::thread_rng()
          .sample_iter(Uniform::from(2.0..24.0))
          .take(6)
          .collect();
        sound_waves.set(array);
      }
    }
  });

  rsx! {
    div {
      class: "bg-gray-500 bg-opacity-30 rounded-md w-48",
      onpointerenter: move |_| {
          hovered.set(true);
      },
      onpointerleave: move |_| {
          hovered.set(false);
      },
      div { class: "p-2 w-full",
        img {
          src: image,
          class: "object-cover rounded-md w-full",
          draggable: false
        }
      }
      div { class: "flex flex-row p-2",
        h6 { class: "flex-grow", {name} }
        {
          if hovered() {
            rsx! {}
          } else if playing().0 && current().id() == id {
            rsx! { div {
              class: "flex flex-row items-center gap-1",
              {sound_waves().iter().enumerate().map(|(i, wave)|
                rsx! {
                div {
                  class: "w-0.5 bg-white transition-all duration-500 rounded-full",
                  style: format!("height: {}px;", wave),
                  key: "{i}"
                }
              })}
            }}
          } else {
            rsx! {}
          }
        }
      }
    }
  }
}
