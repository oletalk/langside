fn main() {
  use std::cell::RefCell;
  use std::cell::Cell;

  struct Point {
    x: i32,
    y: Cell<i32>,
  }

  let point = Point { x: 5, y: Cell::new(6) };

  point.y.set(7);
  println!("y: {:?}", point.y);
  println!("internal value of y is {}", point.y.get());


  let x = RefCell::new(42);

  let y = x.borrow_mut();
  let z = x.borrow_mut();
  // panics:
  //    thread 'main' panicked at 'already borrowed: BorrowMutError', ../src/libcore/result.rs:799
  //    note: Run with `RUST_BACKTRACE=1` for a backtrace.

  // mutability is prop of either a borrow (&mut foo) or a binding (let mut foo)
  // can't have a struct with mix of mutable&non-mutable

}
