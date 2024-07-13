use std::{future::Future, path::PathBuf, time::Duration};

use super::lyrics::{LyricLine, Lyrics, RawLyrics};

#[derive(Clone)]
pub struct Song {
  pub id: i32,
  pub name: String,
  pub artist: String,
  pub duration: Option<Duration>,
  pub location: PathBuf,
  pub image: Option<PathBuf>,
  pub lyrics: Option<PathBuf>,
}

impl Song {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn artist(&self) -> &str {
    &self.artist
  }

  pub fn duration(&self) -> Option<Duration> {
    self.duration
  }

  pub fn location(&self) -> &PathBuf {
    &self.location
  }

  pub fn lyrics(&self) -> Option<&PathBuf> {
    self.lyrics.as_ref()
  }

  pub async fn lyrics_content(&self) -> Result<Lyrics, String> {
    if self.lyrics.is_none() {
      let raw_lyrics = reqwest::get(format!(
        "https://lrclib.net/api/search?track_name={}",
        self.name
      ))
      .await
      .expect("Failed to fetch lyrics.")
      .json::<Vec<RawLyrics>>()
      .await
      .expect("Failed to parse lyrics.");

      let mut lyrics = vec![];
      let raw_lyric = raw_lyrics.get(0);
      if let None = raw_lyric {
        return Err("No lyrics found".to_string());
      }
      let raw_lyric = raw_lyric.unwrap().clone();
      let lines = raw_lyric.synced_lyrics();
      if let None = lines {
        return Err("No lyrics found".to_string());
      }
      let lines = lines.unwrap();
      let lines = lines.split('\n');

      let mut first = true;
      for line in lines.rev().into_iter() {
        let mut parts = line.split(']');
        // parse 03:06.57 etc
        let time = parts
          .next()
          .unwrap()
          .split('[')
          .last()
          .unwrap()
          .split(':')
          .map(|s| s.parse::<f64>().unwrap())
          .collect::<Vec<f64>>();

        let time = time[0] * 60.0 + time[1];

        let content = parts.next().unwrap().trim().to_string();

        if first {
          first = false;
          lyrics.push(LyricLine::new(
            time,
            self.duration.map(|d| d.as_secs_f64()).unwrap_or(time),
            content,
          ));
        } else {
          let last = lyrics.last().unwrap();
          let duration = last.start() - time;
          lyrics.push(LyricLine::new(time, duration, content));
        }
      }

      lyrics.reverse();

      return Ok(lyrics);
    }
    Err("No lyrics found".to_string())
  }

  pub fn image(&self) -> Option<&PathBuf> {
    self.image.as_ref()
  }
}

impl PartialEq for Song {
  fn eq(&self, other: &Self) -> bool {
    self.id() == other.id()
  }
}
impl Eq for Song {}

#[derive(Clone)]
pub struct CurrentTime(pub f64);

#[derive(Clone)]
pub struct Playing(pub bool);
