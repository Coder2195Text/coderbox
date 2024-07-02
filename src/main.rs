mod components;
mod db;
mod pages;
mod structs;

use std::{fs::File, io::BufReader, path::PathBuf, sync::RwLock, thread, time::Duration};

use crate::pages::home::Home;
use crate::pages::lyrics::Lyrics;
use crate::structs::song::Song;
use components::ui::layout::Layout;
use db::setup::setup_database;
use dioxus::{desktop::WindowBuilder, prelude::*};
use once_cell::sync::Lazy;
use rodio::{Decoder, OutputStream, Sink};
use structs::{
  lyrics::LyricLine,
  playlist::Playlist,
  song::{CurrentTime, Playing},
};
use tracing::Level;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
  #[layout(Layout)]
  #[route("/")]
  Home {},

  #[route("/lyrics")]
  Lyrics {},
}

static DATABASE_URL: Lazy<PathBuf> =
  Lazy::new(|| dir::home_dir().unwrap().join("Music/coderbox.db"));
static SINK_INSTANCE: RwLock<Option<Sink>> = RwLock::new(None);

fn main() {
  setup_database().expect("failed to setup database");

  thread::spawn(move || {
    let (_stream, stream_handle) =
      OutputStream::try_default().expect("failed to get output stream");
    let sink = Sink::try_new(&stream_handle).expect("failed to create sink");

    SINK_INSTANCE.write().unwrap().replace(sink);

    let file = BufReader::new(File::open("/home/coder2195/Music/rickroll.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    let sink_instance = SINK_INSTANCE.read().unwrap();
    let sink = sink_instance.as_ref().unwrap();
    sink.append(source);
    sink.pause();

    loop {
      // keep the thread alive to keep the audio playing
      thread::sleep(Duration::from_secs(1000))
    }
  });

  dioxus_logger::init(Level::INFO).expect("failed to init logger");

  let cfg = dioxus::desktop::Config::new()
    .with_custom_head("<link rel=\"stylesheet\" href=\"public/tailwind.css\">".to_string())
    .with_window(WindowBuilder::new().with_title("Coderbox"));

  LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
  let home = dir::home_dir().expect("failed to get home dir");
  let path = home.join("Music/rickroll.mp3");

  use_context_provider::<Signal<Option<Song>>>(|| {
    Signal::new(
      Song {
        name: "Never Gonna Give You Up".to_string(),
        artist: "Rick Astley".to_string(),
        duration: mp3_duration::from_path(path.clone()).ok(),
        location: path,
        lyrics: None,
        image: None,
        id: "1".to_string(),
      }
      .into(),
    )
  });
  use_context_provider(|| Signal::new(CurrentTime(0.0)));
  use_context_provider(|| Signal::new(Playing(false)));
  use_context_provider(|| Signal::new(Playlist::MySongs));

  rsx! {
    Router::<Route> {}
  }
}

#[component]
fn Error(route: Vec<String>) -> Element {
  rsx! {
    div { "err" }
  }
}
