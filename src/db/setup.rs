use rusqlite::Connection;

use crate::DATABASE_URL;

pub fn setup_database() -> Result<(), rusqlite::Error> {
  let root = DATABASE_URL.to_path_buf();
  let connection = Connection::open(root)?;

  connection.execute(
    "
    CREATE TABLE IF NOT EXISTS songs (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL,
      artist TEXT NOT NULL,
      duration INTEGER,
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
