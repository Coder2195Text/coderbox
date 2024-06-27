use core::time;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use dioxus::prelude::*;
use rodio::Decoder;

use crate::components::icons::fa::FaMicrophone;
use crate::components::icons::fa::FaRegCirclePause;
use crate::components::icons::fa::FaRegCirclePlay;
use crate::components::icons::io::IoVolumeHighOutline;
use crate::components::icons::io::IoVolumeLowOutline;
use crate::components::icons::io::IoVolumeMediumOutline;
use crate::components::icons::io::IoVolumeOffOutline;
use crate::structs::song::Song;
use crate::Route;
use crate::SINK_INSTANCE;

#[component]
pub fn Player() -> Element {
  let mut held = use_signal(|| false);
  let mut playing = use_signal(|| false);
  let mut volume = use_signal(|| 100.0);

  let mut current_time = use_context::<Signal<f64>>();
  let current_song = use_context::<Signal<Option<Song>>>();

  let song = current_song();
  let route = use_route::<Route>();

  let _progress_task: Coroutine<()> = use_coroutine(|_| async move {
    loop {
      tokio::time::sleep(time::Duration::from_millis(200)).await;
      if held() || !playing() {
        continue;
      }
      current_time.set(current_time() + 0.2);
    }
  });

  let sink_instance: std::sync::RwLockReadGuard<Option<rodio::Sink>> =
    SINK_INSTANCE.read().unwrap();
  let sink = sink_instance.as_ref().unwrap();
  if playing() {
    sink.play();
  } else {
    sink.pause();
  }

  let duration = if let Some(song) = current_song.as_ref() {
    song.duration()
  } else {
    None
  };

  rsx! {
    div { class: "m-2 rounded-md gradient-bg p-2 gap-2 flex",
      div { class: "w-1/4 px-2",
        {
          song.as_ref().map(|song| song.name()).unwrap_or("Not Playing")
        },
        br {}
        {
          song.as_ref().map(|song| song.artist()).unwrap_or("")

        }
      }
      div { class: "w-1/2 rounded-full flex flex-col",
        input {
          r#type: "range",
          min: 0,
          step: 0.2,
          max: duration.map(|d| d.as_secs()).unwrap_or(0) as f64,
          value: current_time(),
          class: "w-full",
          onmousedown: move |_| {
              held.set(true);
          },
          onmouseup: move |_| {
              let sink_instance = SINK_INSTANCE.read().unwrap();
              let sink = sink_instance.as_ref().unwrap();
              let current = current_time();
              if sink.empty() {
                  if let Some(song) = &song {
                      let source = Decoder::new(
                              BufReader::new(File::open(song.location()).unwrap()),
                          )
                          .unwrap();
                      sink.append(source);
                  }
              }
              sink.try_seek(Duration::from_secs(current as u64)).ok();
              held.set(false);
          },
          oninput: move |e| {
              current_time.set(e.value().parse().expect("not number"));
          }
        }
        div { class: "flex pt-1 justify-between flex-grow items-start",
          div { class: "text-sm w-1/4",
            {
              let val = current_time();
              format!("{:02}:{:02}", val as u64 / 60, val as u64 % 60)
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
          div { class: "text-sm bold w-1/4 text-right",
            {
              if let Some(duration) = duration {
                format!("{:02}:{:02}", duration.as_secs() / 60, duration.as_secs() % 60)
              } else {
                "--:--".to_string()
              }
            }
          }
        }
      }
      div { class: "w-1/4 flex flex-col px-2",
        span { class: "flex gap-1 justify-center",
          Link {
            to: Route::Lyrics {},
            class: {
                let highlight = if (route == Route::Lyrics {}) { "text-teal-500" } else { "" };
                format!("{}", highlight)
            },
            FaMicrophone { size: "24px" }
          }
        }
        span { class: "flex gap-1 flex-grow items-center",
          {
            let val = volume();
            if val == 0.0 {
              rsx! {
                IoVolumeOffOutline {}
              }
            }
            else if val < 100.0/3.0 {
              rsx! {
                IoVolumeLowOutline {}
              }
            }
            else if val < 100.0/3.0*2.0 {
              rsx! {
                IoVolumeMediumOutline {}
              }
            }
            else {
              rsx! {
                IoVolumeHighOutline {}
              }
            }
          },
          input {
            r#type: "range",
            min: 0,
            max: 100,
            value: volume(),
            class: "flex-grow",
            style: "--g:3px;--l:3px;--s:12px;",
            oninput: move |e| {
                let sink_instance = SINK_INSTANCE.read().unwrap();
                let sink = sink_instance.as_ref().unwrap();
                let val = e.value().parse::<f32>().expect("not number");
                sink.set_volume(val / 100.0);
                volume.set(val.into());
            }
          }
        }
      }
    }
  }
}
