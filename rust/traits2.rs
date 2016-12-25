use std::io::Write; // explicitly 'use' the Write trait to be able to use 'write' below

fn main() {

  let mut f = std::fs::File::create("foo.txt").expect("Couldn't create foo.txt");
  let buf = b"whatever";
  let result = f.write(buf);
}
