#[derive(Clone,Hash,Debug)]
pub struct Song {
    pub hash    : String,
    pub artist  : String,
    pub title   : String,
    pub secs    : i32
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
