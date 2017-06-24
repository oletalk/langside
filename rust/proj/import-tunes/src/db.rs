use song::Song;
use consts::db::{ALL_SONGS, DBURL, DB_TEST, FIND_PLAYLIST_NAME};

use postgres::{Connection, TlsMode};
use postgres::rows::Row;
use std::collections::HashMap;

pub fn get_playlist_id(playlist_name: &str) -> Option<i32> {
    let mut ret : Option<i32> = None;
    let conn = match Connection::connect(DBURL, TlsMode::None) {
        Ok(c) => c,
        Err(e) => panic!("Error connecting to the database: {:?}", e),
    };

    for row in &conn.query(FIND_PLAYLIST_NAME, &[&playlist_name]).unwrap() {
        ret = Some(row.get("id"));
    }
    ret

}

pub fn all_songs() -> HashMap<String, Song> {
    fetch_songs(ALL_SONGS, None)
}

pub fn fetch_songs(sql: &str, playlist_name: Option<String>) -> HashMap<String, Song> {
    let conn = match Connection::connect(DBURL, TlsMode::None) {
        Ok(c) => c,
        Err(e) => panic!("Error connecting to the database: {:?}", e),
    };

    let mut songs = HashMap::new();

    match playlist_name {
        Some(n) => {
            for row in &conn.query(sql, &[&n]).unwrap() {
                let _path: String = row.get("song_filepath");
                songs.insert(_path, song_from(row));
            }
        },
        None => {
            for row in &conn.query(sql, &[]).unwrap() {
                let _path: String = row.get("song_filepath");
                songs.insert(_path, song_from(row));
            }
        }
    }
    songs
}

/* a poor man's row-mapper */
fn song_from(row: Row) -> Song {      
    let _hash: Option<String> = row.get("file_hash"); // can be null.
    let _artst: Option<String> = row.get("artist");
    /* this can panic if your database data isn't UTF-8 */
    let _ttle: Option<String> = row.get("title");
    let _sec: i32 = row.get("secs");
    
    Song::new(
        string_result(_hash),
        string_result(_artst),
        string_result(_ttle),
        _sec
    )
}

fn string_result(opt: Option<String>) -> String {
    opt.unwrap_or("".to_owned())
}

pub fn test_connection() {
    let conn = Connection::connect(DBURL, TlsMode::None).unwrap();
    
    for row in &conn.query(DB_TEST, &[]).unwrap() {
        // Conversion(WrongType(Int8)) error I got at the start means the below is looking for 8 bytes, or i64!
        let cnt: i64 = row.get(0);
        println!("{}", cnt);
    }

}
