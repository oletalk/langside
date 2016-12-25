fn main() {
  // differences between String and &str (e.g. &str is static, immutable)
  let hello = "Hello ".to_string(); // String
  let world = "world!"; // &str
  //let world = "world!".to_string(); // String

  let hello_world = hello + &world; // don't need & if world is an &str
  println!("
      {}", hello_world);
}
