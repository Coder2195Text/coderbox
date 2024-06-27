use std::path::PathBuf;

use rusqlite::Connection;

use crate::{structs::song::Song, DATABASE_URL};

pub fn get_all_songs() -> Vec<Song> {
  let connection = Connection::open(DATABASE_URL.to_path_buf()).unwrap();
  todo!()
}
