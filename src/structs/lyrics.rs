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
