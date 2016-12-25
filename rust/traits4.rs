trait ConvertTo<Output> {
  fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
  fn convert(&self) -> i64 { *self as i64 }
}

fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
  x.convert()
}

fn inverse<T>(x: i32) -> T
    // this is using ConvertTo as if it were "ConvertTo<i64>"
    where i32: ConvertTo<T> {
  x.convert()
}

fn main() {
  //let &x = &32;
  let y = 32;
  println!("{}", normal(&y));
  println!("{}", inverse(y));
}
