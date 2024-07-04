use std::collections::HashMap;

use super::{playlist::Playlist, song::Song};

#[derive(Clone)]

pub struct MusicDB {
  pub songs: HashMap<i32, Song>,
  playlists: HashMap<i32, Playlist>,
  playlist_contents: HashMap<i32, Vec<i32>>,
  song_playlists: HashMap<i32, Vec<i32>>,
}

impl MusicDB {
  pub fn new() -> Self {
    let mut obj = Self {
      songs: HashMap::new(),
      playlists: HashMap::new(),
      playlist_contents: HashMap::new(),
      song_playlists: HashMap::new(),
    };
    obj.add_playlist(Playlist::MySongs);
    obj
  }

  pub fn add_song(&mut self, song: Song) {
    let id = song.id();
    self.songs.insert(id, song);
    self.add_song_to_playlist(id, 0);
  }

  pub fn get_song(&self, id: i32) -> Option<&Song> {
    self.songs.get(&id)
  }

  pub fn add_playlist(&mut self, playlist: Playlist) {
    self.playlists.insert(playlist.id(), playlist);
  }

  pub fn get_playlist(&self, id: i32) -> Option<&Playlist> {
    self.playlists.get(&id)
  }

  pub fn add_song_to_playlist(&mut self, song_id: i32, playlist_id: i32) {
    self
      .playlist_contents
      .entry(playlist_id.into())
      .or_insert_with(Vec::new)
      .push(song_id.into());

    self
      .song_playlists
      .entry(song_id.into())
      .or_insert_with(Vec::new)
      .push(playlist_id.into());
  }

  pub fn get_playlist_contents(&self, playlist_id: i32) -> Option<&Vec<i32>> {
    self.playlist_contents.get(&playlist_id)
  }

  pub fn get_song_playlists(&self, song_id: i32) -> Option<&Vec<i32>> {
    self.song_playlists.get(&song_id)
  }
}
