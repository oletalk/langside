use postgres::{Connection, TlsMode};
use postgres::rows::Row;
use std::collections::HashMap;

// TODO: put in config file
static DBURL: &'static str = "postgresql://web@192.168.0.4/maindb";

// SQL QUERIES
static ALL_SONGS: &'static str = "SELECT song_filepath, file_hash, artist, title, secs FROM mp3s_tags";
static SPECIFIC_PLS: &'static str = " t 
JOIN mp3s_playlist_song ps ON (t.file_hash = ps.file_hash)
JOIN mp3s_playlist p ON (p.id = ps.playlist_id)
AND p.name = ";
static DB_TEST: &'static str = "SELECT count(*) FROM mp3s_tags";

#[derive(Clone,Hash,Debug)]
pub struct Song {
    hash    : String,
    artist  : String,
    title   : String,
    secs    : i32
}

impl Song {
    fn new(ihash: String, iartist: String, ititle: String, isecs: i32) -> Song {
        Song {
            hash: ihash,
            artist: iartist,
            title: ititle,
            secs: isecs
        }
    }
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

/* fn songs_for_playlist(pls_name: &str) -> HashMap<String, Song> {
    let sql = [ ALL_SONGS, SPECIFIC_PLS, "'", pls_name , "'" ].concat();
    fetch_songs(sql)
} */


fn all_songs() -> HashMap<String, Song> {
    fetch_songs(ALL_SONGS, None)
}

fn fetch_songs(sql: &str, playlist_name: Option<String>) -> HashMap<String, Song> {
    let conn = Connection::connect(DBURL, TlsMode::None).unwrap();
    let mut songs = HashMap::new();

    match playlist_name {
        Some(n) => { /* FIXME: this needs to be a parametrised query using the playlist name n */
            for row in &conn.query(sql, &[]).unwrap() {
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
    opt.unwrap_or("".to_string())
}

pub fn test_connection() {
    let conn = Connection::connect(DBURL, TlsMode::None).unwrap();
    
    for row in &conn.query(DB_TEST, &[]).unwrap() {
        // Conversion(WrongType(Int8)) error I got at the start means the below is looking for 8 bytes, or i64!
        let cnt: i64 = row.get(0);
        println!("{}", cnt);
    }

}