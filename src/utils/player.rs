use std::{
  fs::File,
  io::BufReader,
  time::{Duration, Instant},
};

use dioxus::signals::{Readable, Signal, Writable};
use rodio::Decoder;

use crate::{
  structs::song::{CurrentTime, Playing, Song},
  SINK_INSTANCE, TIME,
};

pub fn set_play(
  song: Song,
  mut current_song: Signal<Option<Song>>,
  time: Option<f64>,
  current_time: Option<Signal<CurrentTime>>,
  playing: Option<Signal<Playing>>,
  play: Option<bool>,
) {
  let sink_instance = SINK_INSTANCE.read().unwrap();
  let sink = sink_instance.as_ref().unwrap();

  let current = current_song.read().clone();

  if current.map(|c| c.id()).unwrap_or(-1) != song.id() {
    current_song.write().replace(song.clone().into());

    sink.clear();
  }

  if sink.empty() {
    let source = Decoder::new(BufReader::new(File::open(song.location()).unwrap())).unwrap();
    sink.append(source);
  }

  if let Some(time) = time {
    sink.try_seek(Duration::from_secs_f64(time)).unwrap();
    current_time.unwrap().write().0 = time;
  }

  if let Some(play) = play {
    let mut playing = playing.unwrap();
    playing.set(Playing(play));
    if play {
      *TIME.write().unwrap() = Instant::now();
      sink.play();
    } else {
      current_time.unwrap().write().0 += TIME.read().unwrap().elapsed().as_secs_f64();
      sink.pause();
    }
  }
}
