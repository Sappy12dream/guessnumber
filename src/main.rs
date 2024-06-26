use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please enter your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess_num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number:");
                continue;
            }
        };
        println!("You guessed: {}", guess_num);

        match guess_num.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too Small!"),
            std::cmp::Ordering::Greater => println!("too Big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
