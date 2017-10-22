extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  // println!("hello world");
  // let x = 90.456;
  // println!("this is x {}", x);
  // guessing();
  let x = sum(5,6);
  println!("{}", x);
  rev_print_nums();
  simple_iteration_print(&[1, 2, 3, 5]);
}

fn sum(a: u32, b: u32) -> u32 {
  a+b
}

fn simple_iteration_print(a: &[u32]) {
  for elem in a {
    println!("element is {} ",elem);
  }
}

fn rev_print_nums() {
  for elem in (1..90).rev() {
    println!("element printed is {}", elem);
  }
}

fn guessing() {
  let secret_number = rand::thread_rng().gen_range(1, 101);
  println!("Guess the number");

  // println!("secret number is {}", secret_number);

  loop {
    println!("Enter your number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read Line");
    println!("You guessed the number {}", guess);
    let mut guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

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