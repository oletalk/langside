use song::Song;
use consts::db::{
    ALL_SONGS, 
    DBURL, DB_TEST, DEL_PLAYLIST_CONTENTS,
    FIND_PLAYLIST_NAME, 
    INS_PLAYLIST_NAME, INS_PLAYLIST_CONTENTS
};

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

pub fn save_playlist_header(playlist_name : &str) -> i32 {
    let conn = Connection::connect(DBURL, TlsMode::None)
        .expect("Error connecting to the database");
    

    //save new row
    conn.execute(INS_PLAYLIST_NAME, &[&playlist_name])
        .expect("Couldn't insert playlist name. It may exist under another user in the database");

    //get the id for it
    get_playlist_id(playlist_name).unwrap()
}

// TODO: check if a SYSTEM playlist, don't want to delete other users' playlists
pub fn save_playlist_contents(playlist_id : i32, playlist_contents : Vec<Song> ) {
    let conn = match Connection::connect(DBURL, TlsMode::None) {
        Ok(c) => c,
        Err(e) => panic!("Error connecting to the database: {:?}", e),
    };

    let trans = match conn.transaction() {
        Ok(txn) => txn,
        Err(e) => panic!("Unable to start transaction: {:?}", e),    
    }; 

    // remove any existing songs associated with this playlist id
    let del = match trans.execute(DEL_PLAYLIST_CONTENTS, &[&playlist_id]) {
        Ok(c) => c,
        Err(e) => panic!("Error removing previous playlist contents: {:?}", e),
    };

    println!("playlist contained {} rows which were deleted", del);
    // insert songs
    for song in &playlist_contents {
        match trans.execute(INS_PLAYLIST_CONTENTS, &[&playlist_id, &(song.hash)]) {
            Ok(_) => {},
            Err(e) => panic!("Couldn't insert playlist row: {:?}", e),

        };
        // can't seem to use '?' (instead of full-blown match) at the end here, 
        // i get "the trait `std::ops::Carrier` is not implemented for `()`"
    }
    match trans.commit() {
        Ok(_) => {
            println!("Successfully committed playlist of {} rows.", playlist_contents.len())
        },
        Err(e) => panic!("Couldn't commit the transaction: {:?}", e),
    }
    // "expected type () found type std::result::Result<... etc etc" means Rust thought you wanted to return the above expression...

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
