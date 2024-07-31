use std::{path::PathBuf, time::Duration};

use dioxus::prelude::*;
use dioxus_free_icons::{
  icons::fa_regular_icons::{FaCirclePause, FaCirclePlay},
  Icon,
};

use crate::{
  structs::{
    music_db::MusicDB,
    playlist::Playlist,
    song::{CurrentTime, Playing, Song},
  },
  utils::player::set_play,
};
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
  let mut image = use_signal(|| {
    image
      .map(|image| image.to_string_lossy().to_string())
      .unwrap_or("./public/cover.png".into())
  });

  let mut hovered = use_signal(|| false);

  let mut current_playlist = use_context::<Signal<Playlist>>();
  let current_song = use_context::<Signal<Option<Song>>>();
  let current_time = use_context::<Signal<CurrentTime>>();

  let db = use_context::<Signal<MusicDB>>();

  let playing = use_context::<Signal<Playing>>();

  let mut sound_waves = use_signal(|| vec![2.0; 6]);

  let moved_id = id.clone();

  let _task: Coroutine<()> = use_coroutine(|_| async move {
    loop {
      tokio::time::sleep(Duration::from_millis(500)).await;
      if playing().0 && current_playlist().id() == moved_id {
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
      class: "bg-gray-500 bg-opacity-30 rounded-md relative",
      onpointerenter: move |_| {
          hovered.set(true);
      },
      onpointerleave: move |_| {
          hovered.set(false);
      },
      div { class: "m-2 w-48 h-48 rounded-md overflow-hidden flex",
        img {
          class: "object-cover",
          src: image(),
          draggable: false,
          onerror: move |_| {
              image.set("./public/cover.png".into());
          }
        }
      }
      div { class: "p-1 pl-2 pr-9 h-9",
        {
          if hovered() && db.read().get_playlist_contents(id).map(|x| !x.is_empty()).unwrap_or(false) { rsx!(
          button {
            class: "w-8 h-8 absolute bottom-1 right-1",
            onclick: move |e| {
              e.stop_propagation();
              let db = db.read();

              if current_playlist.read().id() != id {
                let song = db.get_playlist_contents(id).and_then(|x| db.get_song(x.first().unwrap().to_owned()).cloned()).unwrap();

                current_playlist.set(db.get_playlist(id).cloned().unwrap_or(Playlist::MySongs));


                set_play(song, current_song, 0.0.into(), current_time.into(), playing.into(), true.into())
              } else {
                if let Some(song) = current_song() {
                  set_play(song, current_song, None, current_time.into(), playing.into(), (!playing().0).into());
                }
              }
            },
            {
              if playing().0 && current_playlist().id() == id {
                rsx! (Icon { icon: FaCirclePause, class: "w-full h-full" } )
              } else {
                rsx! (Icon { icon: FaCirclePlay, class: "w-full h-full" })
            }}
          })
        } else { rsx! {} }},
        {
          if hovered() {
            rsx! {}
          } else if playing().0 && current_playlist().id() == id {
            rsx! { div { class: "w-8 h-8 absolute bottom-1 right-1",  div {
              class: "flex flex-row items-center gap-1 w-full h-full",
              {sound_waves().iter().enumerate().map(|(i, wave)|
                rsx! {
                div {
                  class: "w-0.5 bg-white transition-all duration-500 rounded-full",
                  style: format!("height: {}px;", wave),
                  key: "{i}"
                }
              })}
            }}}
          } else {
            rsx! {}
          }
        },
        {name}
      }
    }
  }
}
