extern crate image;

mod components;
mod db;
mod pages;
mod structs;
mod utils;

use std::{
  fs::File,
  io::BufReader,
  path::PathBuf,
  sync::RwLock,
  thread,
  time::{Duration, Instant},
};

use crate::pages::home::Home;
use crate::pages::lyrics::Lyrics;
use crate::structs::song::Song;
use components::ui::layout::Layout;
use db::setup::{load_db_data, setup_database};
use dioxus::{
  desktop::{tao::window::Icon, WindowBuilder},
  prelude::*,
};
use once_cell::sync::Lazy;
use rodio::{Decoder, OutputStream, Sink};
use structs::{
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
static TIME: Lazy<RwLock<Instant>> = Lazy::new(|| RwLock::new(Instant::now()));

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
    .with_window(
      WindowBuilder::new()
        .with_title("Coderbox")
        .with_maximized(true),
    )
    .with_icon(load_icon("./public/icon.png".into()).into());

  LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
  let db = use_context_provider(|| Signal::new(load_db_data()));

  use_context_provider::<Signal<Option<Song>>>(|| Signal::new(db.read().get_song(1).cloned()));

  use_context_provider(|| Signal::new(CurrentTime(0.0)));
  use_context_provider(|| Signal::new(Playing(false)));
  use_context_provider(|| Signal::new(Playlist::MySongs));

  rsx! {
    Router::<Route> {}
  }
}

fn load_icon(path: PathBuf) -> Icon {
  let (icon_rgba, icon_width, icon_height) = {
    // alternatively, you can embed the icon in the binary through `include_bytes!` macro and use `image::load_from_memory`
    let image = image::open(path)
      .expect("Failed to open icon path")
      .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };
  Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

#[component]
fn Error(route: Vec<String>) -> Element {
  rsx! {
    div { "err" }
  }
}
