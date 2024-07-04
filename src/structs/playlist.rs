use std::path::PathBuf;

use super::song::Song;

#[derive(Clone)]
pub struct PlaylistData {
  pub id: i32,
  pub name: String,
  pub image: Option<PathBuf>,
}

#[derive(Clone)]
pub enum Playlist {
  MySongs,
  Custom(PlaylistData),
}

impl Playlist {
  pub fn id(&self) -> i32 {
    match self {
      Playlist::MySongs => 0,
      Playlist::Custom(data) => data.id,
    }
  }

  pub fn name(&self) -> &str {
    match self {
      Playlist::MySongs => "My Songs",
      Playlist::Custom(data) => &data.name,
    }
  }

  pub fn image(&self) -> Option<&PathBuf> {
    match self {
      Playlist::MySongs => panic!("Don't call image on MySongs"),
      Playlist::Custom(data) => data.image.as_ref(),
    }
  }
}

impl PartialEq for Playlist {
  fn eq(&self, other: &Self) -> bool {
    self.id() == other.id()
  }
}
impl Eq for Playlist {}
