// "extern crate foo" - pull in crates plus our module
extern crate postgres;
extern crate clap;

use clap::App;
mod utils;

// "use"" is just a namespace thing - we don't need it for our module
use postgres::{Connection, TlsMode};
fn main() {
    // 1. Get args - playlist file, name of playlist to store online, whether we are updating an existing playlist
    //     (future - concept of user and whether this user owns the playlist if it is being replaced)
    let matches = App::new("Playlist importer")
                    .version("0.1")
                    .author("Colin M. <oletalk@gmail.com>")
                    .about("Uploads a playlist into the stream server database")
                    .args_from_usage(
                        "-s, --source=[SOURCEFILE] 'Sets the source file containing song file paths'
                         -p, --playlistname=[PLAYLISTNAME] 'Contains the name you want to give the playlist'
                         -u, --update 'Updates the given named playlist'
                         -v, --verbose 'Verbose mode'"
                    )
                    .get_matches();
    let app_options = utils::process_args(&matches);
    println!("{:?}", app_options);
    // 2. Read playlist
    // 3. Open db and find songs in playlist in db (by path)
    // 4. Report on number of playlist songs matching db
    // 5. Check playlist name doesn't yet exist
    // 6. Save entry in playlist table
    // 7. Save playlist songs

    let conn = Connection::connect("postgresql://web@192.168.0.4/maindb", TlsMode::None).unwrap();
    
    for row in &conn.query("SELECT count(*) FROM mp3s_tags", &[]).unwrap() {
        // Conversion(WrongType(Int8)) error I got at the start means the below is looking for 8 bytes, or i64!
        let cnt: i64 = row.get(0);
        println!("{}", cnt);
    }
    
}
