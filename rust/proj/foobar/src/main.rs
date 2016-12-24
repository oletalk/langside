fn main() {
  let x: u8 = 222; // unsigned short
  let y: i32 = 2;
  println!("x is {}, y is {}", x, y);
  foo(y);
  let f = add_one;
  // which is (without type inference) -- let f : fn(i32) -> i32 = add_one;
  println!("function add_one returned {}", f(y));

}

fn foo(x: i32) {
  println!("foo: given number {}", x);
}

fn add_one(x: i32) -> i32 {
  x + 1
}

/* fn diverge() -> ! {
  panic!("I'll never return!");
} */
