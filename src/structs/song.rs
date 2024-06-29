use std::{path::PathBuf, time::Duration};

use super::lyrics::Lyrics;

#[derive(Clone)]
pub struct Song {
  pub id: String,
  pub name: String,
  pub artist: String,
  pub duration: Option<Duration>,
  pub location: PathBuf,
  pub image: Option<PathBuf>,
  pub lyrics: Option<Lyrics>,
}

impl Song {
  pub fn id(&self) -> &str {
    &self.id
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

  pub fn lyrics(&self) -> Option<&Lyrics> {
    self.lyrics.as_ref()
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
pub struct SongData(pub f64, pub bool);
