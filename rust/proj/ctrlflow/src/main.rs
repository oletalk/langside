fn main() {
  let a = true;
  let _y = change_truth(a);
  println!("{}", a);

  let x1 = vec!(23, 34);
  let x2 = vec!(3, 3);
  let (x1, x2) = ownership_demo(x1, x2);
  println!("x2 len is {}", x2.len());
}

fn change_truth(x: bool) -> bool {
  !x
}

fn ownership_demo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
  // etc etc
  (v1, v2)
}
