enum Kingdom { Plant(u32, &'static str), Animal(u32, &'static str) }

fn main() {
  let all_the_big_things = [
    Kingdom::Plant(250, "redwood"),
    Kingdom::Plant(230, "noble fir"),
    Kingdom::Plant(229, "sugar pine"),
    Kingdom::Animal(25, "blue whale"),
    Kingdom::Animal(19, "fin whale"),
    Kingdom::Animal(15, "north pacific right whale"),
  ];

  let mut name_of_biggest_animal = None;
  let mut size_of_biggest_animal= 0;
  for big_thing in &all_the_big_things {
    match *big_thing {
      Kingdom::Animal(size, name) if size > size_of_biggest_animal => {
        size_of_biggest_animal = size;
        name_of_biggest_animal = Some(name);
      }
      Kingdom::Animal(..) | Kingdom::Plant(..) => {}
    }
  }

  match name_of_biggest_animal {
    Some(name) => println!("the biggest animal is {}", name),
    None => println!("there are no animals..."),
  }
}
