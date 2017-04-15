// "extern crate foo" - pull in crates plus our module
extern crate clap;
extern crate postgres;

use clap::App;
mod utils;
mod db;

// "use"" is just a namespace thing - we don't need it for our module
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

    db::test_connection();
    let playlist = vec![
        "/opt/gulfport/mp3/ripped/James_Newton_Howard_-_San_Francisco.mp3".to_string(), 
        "/opt/gulfport/mp3/vartmpnapshare/Amy Winehouse - What It Is About Men.mp3".to_string(),
        "/opt/gulfport/mp3/vartmpnapshare/Billy Currington - 06 - Where The Girls Are.mp3".to_string(),
        "/opt/gulfport/mp3/vartmpnapshare/D12 _ Eminem - Purple Pills.mp3".to_string(),
        "/opt/gulfport/mp3/vartmpnapshare/Jennifer Lopez - Ain't It Funny.mp3".to_string()
    ];
    let result = db::get_songs(playlist);
    for song in result {
        println!("{:?}", song);
    }
    // 2. Read playlist
    // 3. Open db and find songs in playlist in db (by path)
    // 4. Report on number of playlist songs matching db
    // 5. Check playlist name doesn't yet exist
    // 6. Save entry in playlist table
    // 7. Save playlist songs

    
}
