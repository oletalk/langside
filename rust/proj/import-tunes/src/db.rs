use postgres::{Connection, TlsMode};
use std::collections::HashMap;

// TODO: put in config file
static DBURL: &'static str = "postgresql://web@192.168.0.4/maindb";

#[derive(Clone,Hash,Debug)]
pub struct Song {
    hash    : String,
    artist  : String,
    title   : String,
    secs    : i32
}

pub fn get_songs(filenames: Vec<String>) -> Vec<Song> {
    let songs: HashMap<String, Song> = all_songs();
    let mut ret: Vec<Song> = Vec::new();
    //go through filenames, checking if the value is a key in songs
    for f in filenames {
        if songs.contains_key(&f) {
            match songs.get(&f) {
                Some(s) => { ret.push(s.clone()); } // get a copy into ret...
                None => {}
            }
        }
    }
    ret
}

fn all_songs() -> HashMap<String, Song> {
    let conn = Connection::connect(DBURL, TlsMode::None).unwrap();
    let mut songs = HashMap::new();
    for row in &conn.query("SELECT song_filepath, file_hash, artist, title, secs FROM mp3s_tags", &[]).unwrap() {
        let _path: String = row.get("song_filepath");
        let _hash: Option<String> = row.get("file_hash"); // can be null.
        let _artst: Option<String> = row.get("artist");
        /* thread 'main' panicked at 'error retrieving column "artist": 
               Conversion(Utf8Error { valid_up_to: 5 })', 
               /Users/colin/.cargo/registry/src/github.com-1ecc6299db9ec823/postgres-0.14.0/src/rows.rs:206 
        */
        let _ttle: Option<String> = row.get("title");
        let _sec: i32 = row.get("secs");
        songs.insert(_path, Song{
            hash: string_result(_hash),
            artist: string_result(_artst),
            title: string_result(_ttle),
            secs: _sec
        });
    }
    songs
}

fn string_result(opt: Option<String>) -> String {
    opt.unwrap_or("".to_string())
}

pub fn test_connection() {
    let conn = Connection::connect(DBURL, TlsMode::None).unwrap();
    
    for row in &conn.query("SELECT count(*) FROM mp3s_tags", &[]).unwrap() {
        // Conversion(WrongType(Int8)) error I got at the start means the below is looking for 8 bytes, or i64!
        let cnt: i64 = row.get(0);
        println!("{}", cnt);
    }

}