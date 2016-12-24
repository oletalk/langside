fn main() {
  struct Point {
    x: i32,
    y: i32,
  }
  enum OptionalTuple {
    Value(i32, i32, i32),
    Missing,
  }
  let x = OptionalTuple::Value(3, 5, 5);
  match x {
    // '..' disregards multiple values, _ disregards one
    OptionalTuple::Value(..) => println!("Got a tuple"),
    OptionalTuple::Missing => println!("No such luck"),
  }

  let tuple: (u32, String) = (5, String::from("hello"));
  let (x, _) = tuple;
  // if above were (x, _s) the println would be an error because part of the tuple
  // was moved to _s
  println!("tuple is: {:?}", tuple);

  let origin = Point { x: 0, y: 0 };

  match origin {
    Point { x, y } => println!("({},{})", x, y)
  }
}
