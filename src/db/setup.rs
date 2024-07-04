use std::path::PathBuf;

use rusqlite::Connection;

use crate::{
  structs::{
    music_db::MusicDB,
    playlist::{Playlist, PlaylistData},
    song::Song,
  },
  DATABASE_URL,
};

pub fn setup_database() -> Result<(), rusqlite::Error> {
  let root = DATABASE_URL.to_path_buf();
  let connection = Connection::open(root)?;

  connection.execute_batch(
    "
    CREATE TABLE IF NOT EXISTS songs (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL,
      artist TEXT NOT NULL,
      location TEXT NOT NULL,
      lyrics TEXT,
      image TEXT
    );
    CREATE TABLE IF NOT EXISTS playlists (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL,
      image TEXT
    );
    CREATE TABLE IF NOT EXISTS playlist_songs (
      playlist_id INTEGER NOT NULL,
      song_id INTEGER NOT NULL,
      FOREIGN KEY (playlist_id) REFERENCES playlists(id),
      FOREIGN KEY (song_id) REFERENCES songs(id),
      PRIMARY KEY (playlist_id, song_id)
    );",
  )?;

  Ok(())
}

pub fn load_db_data() -> MusicDB {
  let root = DATABASE_URL.to_path_buf();
  let connection = Connection::open(root).unwrap();

  let mut db = MusicDB::new();

  let mut query = connection
    .prepare("SELECT id, name, image FROM playlists;")
    .unwrap();

  let query_iter = query
    .query_map([], |row| {
      let path: Option<String> = row.get(2).ok();
      let path = path.map(|path| PathBuf::try_from(path).ok()).flatten();

      Ok(
        (Playlist::Custom(PlaylistData {
          id: row.get(0)?,
          name: row.get(1)?,
          image: path,
        })),
      )
    })
    .unwrap();

  for playlist in query_iter {
    db.add_playlist(playlist.unwrap());
  }

  let mut query = connection
    .prepare("SELECT id, name, artist, location, lyrics, image FROM songs;")
    .unwrap();

  let query_iter = query
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

      Ok(Song {
        id: row.get(0)?,
        name: row.get(1)?,
        artist: row.get(2)?,
        location: location_path,
        lyrics: lyrics_path,
        image: image_path,
        duration,
      })
    })
    .expect("failed to load songs");

  for song in query_iter {
    db.add_song(song.unwrap());
  }

  let mut query = connection
    .prepare("SELECT playlist_id, song_id FROM playlist_songs;")
    .unwrap();

  let query_iter = query
    .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
    .unwrap();

  for item in query_iter {
    let (playlist_id, song_id) = item.unwrap();
    db.add_song_to_playlist(song_id, playlist_id);
  }

  db
}
