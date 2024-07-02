use std::collections::HashMap;

use super::{playlist::Playlist, song::Song};

#[derive(Clone)]

pub struct PlaylistSongs {
  songs: HashMap<String, Song>,
  playlists: HashMap<String, Playlist>,
  playlist_contents: HashMap<String, Vec<String>>,
  song_playlists: HashMap<String, Vec<String>>,
}

impl PlaylistSongs {
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
    let id = song.id().to_string();
    self.songs.insert(id.clone(), song);
    self.add_song_to_playlist(id.as_str(), "my-songs");
  }

  pub fn get_song(&self, id: &str) -> Option<&Song> {
    self.songs.get(id)
  }

  pub fn add_playlist(&mut self, playlist: Playlist) {
    self.playlists.insert(playlist.id().into(), playlist);
  }

  pub fn get_playlist(&self, id: &str) -> Option<&Playlist> {
    self.playlists.get(id)
  }

  pub fn add_song_to_playlist(&mut self, song_id: &str, playlist_id: &str) {
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

  pub fn get_playlist_contents(&self, playlist_id: &str) -> Option<&Vec<String>> {
    self.playlist_contents.get(playlist_id)
  }

  pub fn get_song_playlists(&self, song_id: &str) -> Option<&Vec<String>> {
    self.song_playlists.get(song_id)
  }
}
