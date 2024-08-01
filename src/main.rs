mod db;
mod home;
mod lyrics;
mod structs;
mod ui;
mod utils;

use std::{path::PathBuf, sync::RwLock, thread, time::Duration};

use db::setup::{load_db_data, setup_database};
use dioxus::{desktop::WindowBuilder, prelude::*};
use home::page::Home;
use lyrics::page::Lyrics;
use once_cell::sync::Lazy;
use rodio::{OutputStream, Sink};
use structs::song::Song;
use structs::{
  playlist::Playlist,
  song::{CurrentTime, Playing},
};
use ui::layout::Layout;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
  #[layout(Layout)]
  #[route("/")]
  Home {},

  #[route("/lyrics")]
  Lyrics {},
}

static SINK: RwLock<Option<Sink>> = RwLock::new(None);
static DATA_URL: Lazy<PathBuf> = Lazy::new(|| dirs::data_local_dir().unwrap().join("coderbox"));
static DATABASE_URL: Lazy<PathBuf> = Lazy::new(|| DATA_URL.join("coderbox.db"));

fn main() {
  setup_database().expect("failed to setup database");

  thread::spawn(move || {
    let (_stream, stream_handle) =
      OutputStream::try_default().expect("failed to get output stream");
    let sink = Sink::try_new(&stream_handle).expect("failed to create sink");

    SINK.write().unwrap().replace(sink);

    loop {
      // keep the thread alive to keep the audio playing
      thread::sleep(Duration::from_secs(1000))
    }
  });

  let cfg = dioxus::desktop::Config::new()
    .with_custom_head("<link rel=\"stylesheet\" href=\"../public/tailwind.css\">".to_string())
    .with_window(
      WindowBuilder::new()
        .with_title("Coderbox")
        .with_maximized(true),
    );

  LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
  let db = load_db_data();

  use_context_provider::<Signal<Option<Song>>>(|| Signal::new(db.get_song(1).cloned()));
  use_context_provider(|| Signal::new(db));

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
