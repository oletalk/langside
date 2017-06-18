#[derive(Clone,Hash,Debug)]
pub struct Song {
    hash    : String,
    artist  : String,
    title   : String,
    secs    : i32
}

impl Song {
    pub fn new(ihash: String, iartist: String, ititle: String, isecs: i32) -> Song {
        Song {
            hash: ihash,
            artist: iartist,
            title: ititle,
            secs: isecs
        }
    }
}
