use clap::ArgMatches;

#[derive(Debug)]
pub struct ImportDetails<'a> {
    pub source: &'a str,
    pub playlistname: Option<&'a str>, //&'a str,
    pub update: bool,
    pub verbose: bool
}

// Get args - playlist file, name of playlist to store online, whether we are updating an existing playlist
//     (future - concept of user and whether this user owns the playlist if it is being replaced)
pub fn process_args<'a>(matches: &'a ArgMatches) -> ImportDetails<'a> {
    let source_arg = matches.value_of("source").unwrap();
    let playlistname_arg = matches.value_of("playlistname");
    let update_arg = match matches.occurrences_of("update") {
        0 => false,
        _ => true
    };
    let verbose_arg = match matches.occurrences_of("verbose") {
        0 => false,
        _ => true
    };
    ImportDetails {
        source: source_arg,
        playlistname: playlistname_arg,
        update: update_arg,
        verbose: verbose_arg
    }
}