// SQL QUERIES
pub mod db {
    pub static ALL_SONGS: &'static str = "SELECT t.song_filepath, t.file_hash, t.artist, t.title, t.secs FROM mp3s_tags t";
    pub static SPECIFIC_PLS: &'static str = " JOIN mp3s_playlist_song ps ON (t.file_hash = ps.file_hash)
    JOIN mp3s_playlist p ON (p.id = ps.playlist_id)
    AND p.name = $1";
    pub static DB_TEST: &'static str = "SELECT count(*) FROM mp3s_tags";
    // TODO: put in config file
    pub static DBURL: &'static str = "postgresql://web@192.168.0.4/maindb";
}
