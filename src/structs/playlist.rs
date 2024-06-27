use std::path::PathBuf;

use crate::SONGS;

use super::song::Song;

#[derive(Clone)]
pub struct PlayListData {
  pub id: String,
  pub name: String,
  pub songs: Vec<Song>,
  pub image: Option<PathBuf>,
}

#[derive(Clone)]
pub enum Playlist {
  MySongs,
  Custom(PlayListData),
}

impl Playlist {
  pub fn id(&self) -> &str {
    match self {
      Playlist::MySongs => "my-songs",
      Playlist::Custom(data) => &data.id,
    }
  }

  pub fn name(&self) -> &str {
    match self {
      Playlist::MySongs => "My Songs",
      Playlist::Custom(data) => &data.name,
    }
  }

  pub fn songs(&self) -> &Vec<Song> {
    match self {
      Playlist::MySongs => panic!("Don't call songs on MySongs"),
      Playlist::Custom(data) => &data.songs,
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