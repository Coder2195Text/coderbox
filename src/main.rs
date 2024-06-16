mod components;
mod db;
mod pages;

use std::{fs::File, io::BufReader, sync::RwLock, thread, time::Duration};

use crate::pages::home::Home;
use components::global::layout::Layout;
use dioxus::{desktop::WindowBuilder, prelude::*};
use rodio::{Decoder, OutputStream, Sink};
use tracing::Level;

#[derive(Routable, PartialEq, Clone)]
enum Route {
  #[layout(Layout)]
  #[route("/")]
  Home {},
}

static SINK_INSTANCE: RwLock<Option<Sink>> = RwLock::new(None);

fn main() {
  dioxus_logger::init(Level::INFO).expect("failed to init logger");

  let cfg = dioxus::desktop::Config::new()
    .with_custom_head("<link rel=\"stylesheet\" href=\"public/tailwind.css\">".to_string())
    .with_window(WindowBuilder::new().with_title("Coderbox"));

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

  LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    Router::<Route> {}
  }
}
