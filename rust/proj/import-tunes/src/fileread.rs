use std;
use std::io::Read;
use std::fs::File;
use std::path::Path;

pub fn filecontents(filename: &str) -> Result<String, std::io::Error> {
    let mut data = String::new();
    let mut f = try!(File::open(filename));
    try!(f.read_to_string(&mut data));
    Ok(data)
}

pub fn file_shortname(filename: &str) -> Result<&str,&str> {
    let path = Path::new(filename);
    let ret = path.file_stem();
    match ret {
        Some(os_str) => {
            match os_str.to_str() {
                Some(s) => Ok(s),
                None => Err("OS returned null filename :-(")
            }
        },
        None => Err("no filename was provided")
    }
}
