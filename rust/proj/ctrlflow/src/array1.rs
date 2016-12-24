fn main() {
  let x = 5;

  let y = if x == 5 { 10 } else { 15 }; // y: i32
  assert_eq!(y, 10);

  /* for x in 0..10 {
    println!("{}", x); // x: i32
  } */

  /*
  loop {
    x += x - 3;
    println!("{}", x);

    if x % 5 == 0 { break; }
  }
  */

  let v = vec![4, 6, 2, 3, 2, 5];
  //let v2 = v; // 'moves' the object to v2 so you won't be able to use v anymore
  for i in &v {
    println!("ref - {}", i);
  }

  let v = 1;
  let v2 = v; // primitive is a 'copy' type though so if you move it copies over
  println!("v is {}", v);
}
