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
use structs::{lyrics::LyricLine, playlist::Playlist, song::SongData};
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
static SONGS: RwLock<Vec<Song>> = RwLock::new(vec![]);
static PLAYLISTS: RwLock<Vec<Playlist>> = RwLock::new(vec![]);

fn main() {
  setup_database().expect("failed to setup database");

  let mut playlists = PLAYLISTS.write().unwrap();
  playlists.push(Playlist::MySongs);

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
        lyrics: vec![
          LyricLine::new(0.0, 14.65, "[Music]".to_string()),
          LyricLine::new(18.8, 7.239, "we're no strangers to".to_string()),
          LyricLine::new(21.8, 7.84, "love you know the rules and so do".to_string()),
          LyricLine::new(
            26.039,
            5.201,
            "I I full commitments while I'm thinking".to_string(),
          ),
          LyricLine::new(29.64, 5.88, "of".to_string()),
          LyricLine::new(
            31.24,
            8.2,
            "you wouldn't get this from any other guy".to_string(),
          ),
          LyricLine::new(35.52, 7.84, "I just want to tell you how I'm".to_string()),
          LyricLine::new(
            39.44,
            6.759,
            "feeling got to make you understand Never".to_string(),
          ),
          LyricLine::new(
            43.36,
            6.359,
            "Going To Give You Up never going to let".to_string(),
          ),
          LyricLine::new(
            46.199,
            7.441,
            "you down never going to run around and".to_string(),
          ),
          LyricLine::new(
            49.719,
            6.401,
            "desert you never going to make you cry".to_string(),
          ),
          LyricLine::new(
            53.64,
            7.12,
            "never going to say goodbye never going".to_string(),
          ),
          LyricLine::new(56.12, 7.84, "to tell a lie and hurt you".to_string()),
          LyricLine::new(60.76, 6.56, "we've known each other for so".to_string()),
          LyricLine::new(
            63.96,
            6.64,
            "long your heart's been aching but your".to_string(),
          ),
          LyricLine::new(
            67.32,
            5.2,
            "to sh to say it inside we both know".to_string(),
          ),
        ]
        .into(),
        image: None,
        id: "1".to_string(),
      }
      .into(),
    )
  });
  use_context_provider::<Signal<SongData>>(|| Signal::new(SongData(0.0, true)));
  use_context_provider::<Signal<Playlist>>(|| Signal::new(Playlist::MySongs));

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
