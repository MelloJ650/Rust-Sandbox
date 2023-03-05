//Variables hold primitive data or references to data
//Variable are immutable by default
//Rust is a block-scoped language

pub fn run() {
  let name =  "Joseph";
  let mut age = 24;
  println!("My name is {} and I am {}", name, age);
  age = 25;
  println!("My name is {} and I am {}", name, age);

  //Define Constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  //assign multiple vars
  let (my_name,my_age) = ("JoJo", 26);
  println!("{} is {}", my_name, my_age);


}