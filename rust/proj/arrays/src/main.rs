fn main() {
    let a = [-33, 3, 344, 23, 5, 43, 5];
    println!("a[2] is {}", a[2]);
    println!("a has {} elements", a.len());
    let middle = &a[1..4]; // a[1], a[2], a[3] only
    println!("slice 'middle' has {} elements", middle.len());

    // let x: (i32, &str) = (1, "hello"); // or simply let x = (1, "hello")
    // println'ing it doesn't work
    // 'de-structuring let' -- destructures the tuple and assigns to the three bindings on the lhs
    let (x, y, z) = (1, 2, 3);
    println!("{}", y);

   // tuples
   let (x, y) = get_point(); 
   assert_eq!(x, 4);
   let point = get_point();
   assert_eq!(point.1, 5);
}

fn get_point() -> (i32, i32) {
  (4, 5)
}
