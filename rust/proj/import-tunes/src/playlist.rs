use song::Song;
use consts::db::{ALL_SONGS, SPECIFIC_PLS};
use db::{fetch_songs, all_songs};

use std::collections::HashMap;

pub fn songs_in_playlist(playlist: String) -> Vec<Song> {
    let sql = [ALL_SONGS, SPECIFIC_PLS].join("");
    let song_res = fetch_songs(&sql, Some(playlist));
    let ret: Vec<Song> = song_res.iter().map(|(_, v)| v.clone()).collect();
    ret
}

// Filter all songs in db against given (play)list of filenames
pub fn get_songs(filenames: Vec<String>) -> Vec<Song> {
    let songs: HashMap<String, Song> = all_songs();
    let mut ret: Vec<Song> = Vec::new();

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
