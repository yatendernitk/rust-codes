extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  // println!("hello world");
  // let x = 90.456;
  // println!("this is x {}", x);
  guessing();
}

fn guessing(){
  let secret_number = rand::thread_rng().gen_range(1, 101);
  println!("Guess the number");

  println!("secret number is {}", secret_number);

  loop {
    println!("Enter your number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read Line");

    println!("You guessed the number {}", guess);

    let mut guess: u32 = guess.trim().parse()
      .expect("failed to convert string to interger");

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small.. "),
        Ordering::Greater => println!("Too Big..."),
        Ordering::Equal   => {
          println!("Yaay you guessed it right.. end game");
          break;
        }
    }
  }
}