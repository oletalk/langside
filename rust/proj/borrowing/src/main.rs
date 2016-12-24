
  // if you leave the 'a bits out
  //struct Foo ...and... x: &i32,
  // ... Rust will complain the lifetime specifier is missing
  // see https://doc.rust-lang.org/book/lifetimes.html for details

struct Foo<'a> {
  num: &'a i32,
}

impl<'a> Foo<'a> {
  fn x(&self) -> &'a i32 { self.num }
}

fn main() {
  let y = &5;
  let f = Foo { num: y };

  println!("x is {}", f.x());
}
