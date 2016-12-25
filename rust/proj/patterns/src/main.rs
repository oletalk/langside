fn main() {

  let x = 8;
  match x {
    e @ 1 ... 5 => println!("got a range element {}", e),
    e @ 6 ... 10 => println!("got an element of another range {}", e),
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
    Some(Person { name: None }) => println!("Didn't leave their name..."),
    None  => println!("Nobody..."),
  }

  let msg = None;

  if let Some(ref m) = msg {
    println!("{}", *m);
  } else {
    println!("msg is None...");
  }

  let unwrapped_msg = msg.unwrap_or("default message.");
  println!("{}", unwrapped_msg);
}
