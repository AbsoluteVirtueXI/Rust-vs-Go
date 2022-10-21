use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;
use std::process;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

fn main() {
    let mut count: usize = 0;

    let secret = thread_rng().gen_range(MIN..=MAX);
    if cfg!(debug_assertions) {
        println!("DEBUG: secret = {}", secret);
    }

    loop {
        let mut guess = String::new();
        println!("Turn {}, Please enter a number:", count + 1);
        if let Err(e) = io::stdin().read_line(&mut guess) {
            eprintln!("Error: {}", e);
            process::exit(-1);
        }

        let guess = match guess.trim().parse::<i8>() {
            Ok(nb) => nb,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        count += 1;

        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("Congratulations you won in {} turn.", count);
                break;
            }
            Ordering::Less => {
                println!("{} is too small.", guess);
                continue;
            }
            Ordering::Greater => {
                println!("{} is too big.", guess);
                continue;
            }
        }
    }

    println!("End of game.");
}
