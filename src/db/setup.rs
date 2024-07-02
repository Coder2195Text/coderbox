use std::path::{Path, PathBuf};

use dioxus::html::image;
use rusqlite::Connection;

use crate::{
  components::lyrics,
  structs::{
    playlist::{Playlist, PlaylistData},
    playlist_songs::PlaylistSongs,
    song::Song,
  },
  DATABASE_URL,
};

pub fn setup_database() -> Result<(), rusqlite::Error> {
  let root = DATABASE_URL.to_path_buf();
  let connection = Connection::open(root)?;

  connection.execute(
    "
    CREATE TABLE IF NOT EXISTS songs (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL,
      artist TEXT NOT NULL,
      location TEXT NOT NULL,
      lyrics TEXT,
      image TEXT
    )",
    (),
  )?;

  connection.execute(
    "
    CREATE TABLE IF NOT EXISTS playlists (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL,
      image TEXT
    )",
    (),
  )?;

  Ok(())
}

pub fn load_playlist_songs() -> PlaylistSongs {
  let root = DATABASE_URL.to_path_buf();
  let connection = Connection::open(root).unwrap();

  let mut playlist_songs = PlaylistSongs::new();

  let mut query = connection
    .prepare("SELECT id, name, image FROM playlists")
    .unwrap();

  query
    .query_map([], |row| {
      let path: Option<String> = row.get(2).ok();
      let path = path.map(|path| PathBuf::try_from(path).ok()).flatten();

      playlist_songs.add_playlist(Playlist::Custom(PlaylistData {
        id: row.get(0)?,
        name: row.get(1)?,
        image: path,
      }));

      Ok(())
    })
    .ok();

  let mut query = connection
    .prepare("SELECT id, name, artist, location, lyrics, image FROM songs")
    .unwrap();

  query
    .query_map([], |row| {
      let location_path: String = row.get(3)?;
      let location_path = PathBuf::from(location_path);

      let image_path: Option<String> = row.get(5).ok();
      let image_path = image_path
        .map(|path| PathBuf::try_from(path).ok())
        .flatten();

      let lyrics_path: Option<String> = row.get(4).ok();
      let lyrics_path = lyrics_path
        .map(|path| PathBuf::try_from(path).ok())
        .flatten();

      let duration = mp3_duration::from_path(location_path.as_path()).ok();

      playlist_songs.add_song(Song {
        id: row.get(0)?,
        name: row.get(1)?,
        artist: row.get(2)?,
        location: location_path,
        lyrics: lyrics_path,
        image: image_path,
        duration,
      });

      Ok(())
    })
    .ok();

  playlist_songs
}
