use serde::{Deserialize, Serialize};

pub type Lyrics = Vec<LyricLine>;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct LyricLine {
  start: f64,
  duration: f64,
  content: String,
}

impl LyricLine {
  pub fn new(start: f64, duration: f64, content: String) -> Self {
    Self {
      start,
      duration,
      content,
    }
  }

  pub fn start(&self) -> f64 {
    self.start
  }

  pub fn duration(&self) -> f64 {
    self.duration
  }

  pub fn content(&self) -> &String {
    &self.content
  }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawLyrics {
  pub synced_lyrics: Option<String>,
  pub id: i32,
}

impl RawLyrics {
  pub fn new(synced_lyrics: Option<String>, id: i32) -> Self {
    Self { synced_lyrics, id }
  }

  pub fn synced_lyrics(&self) -> Option<String> {
    self.synced_lyrics.clone()
  }

  pub fn id(&self) -> i32 {
    self.id
  }
}
