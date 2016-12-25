use std::fmt;

struct Circle {
  x: f64,
  y: f64,
  radius: f64,
}

impl Circle {
  fn new(x: f64, y: f64, radius: f64) -> Circle {
    Circle { x: x, y: y, radius: radius }
  }

  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }

  fn grow(&self, increment: f64) -> Circle {
    Circle { x: self.x, y: self.y, radius: self.radius + increment }
  }
}

impl fmt::Debug for Circle { // "implement this Trait for this Type..."
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle {{ x: {}, y: {}, radius: {} }}", self.x, self.y, self.radius)
  }
}

fn main() {
  let c = Circle::new(0.0, 0.0, 2.0);
  // OR let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
  println!("{}", c.area());
  println!("{:?}", c);  // if you want to use :? you need to define a fmt::Debug for the struct

  let d = c.grow(2.0).area();
  println!("{}", d);
}
