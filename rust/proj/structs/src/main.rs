struct Point {
  x: i32,
  y: i32,
}

struct PointRef<'a> {
  x: &'a mut i32,
  y: &'a mut i32,
}

struct Color(i32, i32, i32); // a 'tuple struct'
struct Inches(i32);

fn main() {
  let mut point = Point { x: 0, y: 0 };
  {
    let r = PointRef { x: &mut point.x, y: &mut point.y };

    *r.x = 5;
    *r.y = 6;
  }
  assert_eq!(5, point.x);
  assert_eq!(6, point.y);

  let point2 = Point { y: 2, .. point }; // copies all values in point except y
  assert_eq!(5, point2.x);

  let purple = Color(255, 0, 255);
  assert_eq!(purple.1, 0);

  let length = Inches(10);
  let Inches(integer_length) = length;
  println!("length is {} inches", integer_length);

  let Color(fieldr, fieldg, fieldb) = purple;
  println!("r = {}, g = {}, b = {}", fieldr, fieldg, fieldb);
}
