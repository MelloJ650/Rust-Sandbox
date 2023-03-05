pub fn run() {
  //Print to console
  println!("Hello from the print.rs file!");
  
  //formatted print
  println!("{} is from {}", "Joseph", "Portugal");

  //positional arguments
  println!("{0} doesn't {1} but {0} does {2} a lot", "Joseph", "touch grass", "code");
  
  //Named arguments
  println!("{name} likes to play {videogame}",
   name = "Joseph", 
   videogame = "Chess.com");

  //Placeholder traits
  println!("Binary: {:b} Hex: {:x} Ocatal: {:o}", 10, 10, 10);

  //Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));
}