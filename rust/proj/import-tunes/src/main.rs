// "extern crate foo" - pull in crates plus our module
extern crate clap;
extern crate postgres;

use std::cmp::Ordering;
use std::process;

use clap::App;

pub mod db;
pub mod fileread;
pub mod playlist;
pub mod song;
pub mod utils;
pub mod consts;

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

    // 2. Read playlist (if playlist name not given in -p use file name)
    let playlist_str = match fileread::filecontents(app_options.source) {
        Ok(c) => c,
        Err(e) => {
            println!("Problem reading given playlist: {:?}", e);
            process::exit(1)
        }
    };
    let playlist: Vec<String> = playlist_str.lines().map( |s| s.to_owned()).collect();

    let playlist_num_songs = playlist.len();
    println!("Your playlist has {} song(s)", &playlist_num_songs);

    // 3. Open db and find songs in playlist in db (by path)
    let result = playlist::get_songs(playlist);

    // 4. Report on number of playlist songs matching db
    match result.len().cmp(&playlist_num_songs) {
        Ordering::Equal => println!("All songs matched!"),
        _ => println!("Your playlist matched {} song(s) in the database.", result.len())
    }
    for song in &result {
        println!("{:?}", song);
    }

    let playlist_name = match app_options.playlistname {
        Some(s) => s,
        None => match fileread::file_shortname(app_options.source) {
            Ok(n) => n,
            Err(err) => panic!("Error: {}", err)
        }
    };
    println!("Playlist name will be '{}'", playlist_name);
    

    // 5. Check playlist name doesn't yet exist (or if we're updating with e.g. a batch job use -u)
    // 6. Save entry in playlist table
    // 7. Save playlist songs

    let list_id = match playlist::find_playlist_id(playlist_name.to_owned()) {
        None => {
            match app_options.update {
                false => {
                    // create a playlist if not updating
                    playlist::create_new_playlist(playlist_name)
                },
                true => {
                    // error out if updating (playlist doesn't exist)
                    println!("ERROR: Cannot update a playlist that doesn't exist");
                    process::exit(1)
                }
            }
        },
        Some(id) => {
            match app_options.update {
                false => {
                    // error out if playlist exists but update not specified
                    println!("ERROR: You need to specify update (-u) if playlist already exists");
                    process::exit(1)
                },
                true => { id }
            }
        }
    };

    println!("Will save to playlist name {} id {} ...", playlist_name, list_id);
    playlist::save_playlist(list_id, result);
    // invocation so far: import-tunes -s ~/my-playlist.txt (assuming we are fine with the filename as playlist)
    // TODO - proper logging
}
