use std::{path::PathBuf, time::Duration};

use super::lyrics::{LyricLine, Lyrics};

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

  pub fn lyrics_content(&self) -> Option<Lyrics> {
    vec![
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
    .into()
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
