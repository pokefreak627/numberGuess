use rand::{Rng, prelude::ThreadRng};
use std::io;

fn main() {
    let mut upper = 100; 

    let mut lower = 1;

    let mut rng:ThreadRng = rand::thread_rng();

    loop {
        let guess = rng.gen_range(lower..upper);

        println!("{}", guess);

        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("failed to read line");

            let response = response.trim();

            if response == "too big" {
            upper = guess;
        } else if response == "too small" {
            lower = guess + 1
        } else if response == "correct" {
            println!("huzzah");
            
            break;
        }
    }
}