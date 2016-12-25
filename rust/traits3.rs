use std::fmt::Debug;

// specify everything in first <>
fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
  x.clone();
  y.clone();
  println!("{:?}", y);
}

// use 'where clause' syntax
fn bar<T, K>(x: T, y: K)
  where T: Clone,
        K: Clone + Debug {
  x.clone();
  y.clone();
  println!("{:?}", y);
}

fn main() {
  foo("Hello", "world");
  bar("Hello", "world");
}
