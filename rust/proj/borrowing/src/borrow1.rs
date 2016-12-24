fn main() {

  fn sum_vec(v: &Vec<i32>) -> i32 {
    return v.iter().fold(0, |a, &b| a + b);
  }

  fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let s1 = sum_vec(v1);
    let s2 = sum_vec(v2);

    s1 + s2
  }

  let v1 = vec![4, 3, 2, 4];
  //println!("sum v1 is {}", sum_vec(&v1));
  let v2 = vec![3, 2, 4];
  let answer = foo(&v1, &v2);
  println!("{}", answer);
}
