fn main() {

  let x = 5;
  match x {
    e @ 1 ... 5 => println!("got a range element {}", e),
    _ => println!("anything"),
  }

  struct Person {
    name: Option<String>,
  }

  // more complicated match on part of a data structure
  let name = "Steve".to_string();
  let x: Option<Person> = Some(Person { name: Some(name)} );
  match x {
    Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
      _ => {}
  }
}
