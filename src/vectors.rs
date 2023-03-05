//Vectors - resizable array

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  numbers[2] = 20;

  //Add onto vector
  numbers.push(5);
  numbers.push(6);

  //pop off last value
  numbers.pop();

  println!("{:?}", numbers);

  //get single value
  println!("Single Value: {}", numbers[0]);

  //get vector length
  println!("vector Length: {}", numbers.len());

  //vectors are stack allocated
  println!("vector occupies {} bytes", mem::size_of_val(&numbers));

  //Get slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  //loop through vectors values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  //Loop & Mutate through vector
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers Vec: {:?}", numbers);
}