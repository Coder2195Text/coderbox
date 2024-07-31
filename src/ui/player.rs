use core::time;

use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::FaCirclePlay;
use dioxus_free_icons::icons::fa_solid_icons::FaMicrophone;
use dioxus_free_icons::icons::io_icons::IoVolumeHighOutline;
use dioxus_free_icons::icons::io_icons::IoVolumeLowOutline;
use dioxus_free_icons::icons::io_icons::IoVolumeMediumOutline;
use dioxus_free_icons::icons::io_icons::IoVolumeOffOutline;

use crate::structs::song::CurrentTime;
use crate::structs::song::Playing;
use crate::structs::song::Song;
use crate::utils::player::set_play;
use crate::Route;
use crate::SINK;

use dioxus_free_icons::icons::fa_regular_icons::FaCirclePause;
use dioxus_free_icons::Icon;

#[component]
pub fn Player() -> Element {
  let mut held = use_signal(|| false);
  let mut volume = use_signal(|| 100.0);

  let mut current_time = use_context::<Signal<CurrentTime>>();
  let playing = use_context::<Signal<Playing>>();

  let current_song = use_context::<Signal<Option<Song>>>();
  let song = current_song();

  let image = song.as_ref().map(|song| song.image().cloned()).flatten();

  let mut image = use_signal(|| {
    image
      .map(|image| image.to_string_lossy().to_string())
      .unwrap_or("./public/cover.png".into())
  });

  let route = use_route::<Route>();

  let _progress_task: Coroutine<()> = use_coroutine(|_| async move {
    loop {
      let sink_instance = SINK.read().unwrap();
      let sink = sink_instance.as_ref().unwrap();

      tokio::time::sleep(time::Duration::from_millis(200)).await;
      if held() || !playing().0 {
        continue;
      }

      current_time.write().0 = sink.get_pos().as_secs_f64();
    }
  });

  let duration = if let Some(song) = current_song.as_ref() {
    song.duration()
  } else {
    None
  };

  rsx! {
    div { class: "w-full p-2 fixed bottom-12 md:bottom-0 left-0",
      div { class: "rounded-md gradient-bg gap-2 flex p-2 h-20 w-full",
        div { class: "w-1/3 px-2 h-full flex items-center justify-center flex-col gap-2",
          div { class: "text-xl truncate max-w-full",
            {
              song.as_ref().map(|song| song.name()).unwrap_or("Not playing")
            }
          }
          div { class: "truncate max-w-full",
            {

              song.as_ref().map(|song| song.artist()).unwrap_or("")
            }
          }
        }
        div { class: "w-1/3 rounded-full flex flex-col",
          input {
            r#type: "range",
            min: 0,
            step: 0.2,
            max: duration.map(|d| d.as_secs()).unwrap_or(0) as f64,
            value: current_time().0,
            class: "w-full",
            onmousedown: move |_| {
                held.set(true);
            },
            onmouseup: move |_| {
                let time = current_time().0;
                if let Some(song) = current_song() {
                    set_play(
                        song.clone(),
                        current_song,
                        time.into(),
                        current_time.into(),
                        playing.into(),
                        None,
                    );
                }
                held.set(false);
            },
            oninput: move |e| {
                current_time.write().0 = e.value().parse().unwrap();
            }
          }
          div { class: "flex pt-1 justify-between flex-grow items-start",
            div { class: "text-sm w-1/4",
              {
                let val = current_time().0;
                format!("{:02}:{:02}", val as u64 / 60, val as u64 % 60)
              }
            }
            button {
              class: "mx-auto",
              onclick: move |_| {
                  if let Some(song) = current_song() {
                      set_play(
                          song.clone(),
                          current_song,
                          None,
                          current_time.into(),
                          playing.into(),
                          Some(!playing().0),
                      );
                  }
              },
              {
                if playing().0 {
                  rsx!(
                    Icon {
                      height: 32,
                      width: 32,
                      icon: FaCirclePause
                    }
                  )
                } else {
                  rsx! (
                    Icon {
                      height: 32,
                      width: 32,
                      icon: FaCirclePlay
                    }
                  )
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
        div { class: "w-1/3 flex flex-col px-2",
          span { class: "flex gap-1 justify-center",
            Link {
              to: Route::Lyrics {},
              class: {
                  let highlight = if (route == Route::Lyrics {}) { "text-teal-500" } else { "" };
                  format!("{}", highlight)
              },
              Icon { height: 24, width: 24, icon: FaMicrophone }
            }
          }
          span { class: "flex gap-1 flex-grow items-center px-8",
            {
              let val = volume();
              if val == 0.0 {
                rsx! (
                  Icon {
                    height: 24,
                    width: 24,
                    icon: IoVolumeOffOutline
                  }
                )
              }
              else if val < 100.0/3.0 {
                rsx! (
                  Icon {
                    height: 24,
                    width: 24,
                    icon: IoVolumeLowOutline
                  }
                )
              }
              else if val < 100.0/3.0*2.0 {
                rsx! (
                  Icon {
                    height: 24,
                    width: 24,
                    icon: IoVolumeMediumOutline
                  }
                )
              }
              else {
                rsx! (
                  Icon {
                    height: 24,
                    width: 24,
                    icon: IoVolumeHighOutline
                  }
                )
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
                  let val = e.value().parse::<f32>().expect("not number");
                  let sink_instance = SINK.read().unwrap();
                  let sink = sink_instance.as_ref().unwrap();
                  sink.set_volume(val / 100.0);
                  volume.set(val.into());
              }
            }
          }
        }
      }
    }
  }
}
