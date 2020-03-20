use rand::Rng;
use std::cmp::Ordering; // Ordering is an enum
use std::io; // Rng is a trait

fn main() {
    println!("===== WELCOME TO THE GUESSING GAME =====");
    let mut steps: u32 = 0;
    let secret = rand::thread_rng().gen_range(1, 101); // generate a number in [1, 100]

    println!("Computer has a secret number in mind between 1 and 100, try to guess it in fewest steps possible");

    loop {
        let mut guess = String::new();

        io::stdin() // retuns io::Stdin object
            .read_line(&mut guess) // returns io::Result object
            .expect("Failed to get the guess");

        // here | guess.trim().parse()  | returns a Result which is an enum
        // that has only 2 values : Ok and Err...
        // and since in Rust blocks are evaluated into values, we can use a match statement to ignore error cases
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        steps += 1;
        // |  guess.cmp(...)  | returns an Ordering which is an enum
        // that has 3 values : Greater, Less, Equal
        match guess.cmp(&secret) {
            Ordering::Greater => println!("{} is greater!", guess),
            Ordering::Less => println!("{} is smaller!", guess),
            Ordering::Equal => {
                println!("Bingo!");
                evaluate_success(steps);
                break;
            }
        }
    }
}

/*
Evaluates the success rate
*/
fn evaluate_success(steps: u32) {
    print!("Success status : ");
    match steps {
        1 => println!("Legendary"),
        2 => println!("Awesome!"),
        3 => println!("Nice!"),
        4 => println!("Good!"),
        _ => println!("Not bad"),
    }
    println!("It took you {} steps", steps);
}
