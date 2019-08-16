// extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,11);
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Fail  to read line");
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(err) => {
                println!("输入有误，请重新输入!{}",err);
                continue
            }
        };
        println!("You guessed: '{}'", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("MATCH SUCCESS!");
                break;
            }
        }
        println!("secret_number value is :{} ", secret_number);
    }
}