use rand::Rng;
use std::io;
use colored::*;

fn main() {
    let mut stick_count = 21;
    let mut your_pick_sum = 0;

    println!("{}", "<========================21 Sticks========================>\nRules are\nEach player can draw 1 or 2 sticks at a time\nThe one who draws the last one loses\nAny input other than a number will be considered as 1".yellow());

    draw(&stick_count, &your_pick_sum);

    loop {
        match handle_user(&stick_count, &your_pick_sum) {
            Some(pair) => {
                let (a, b) = pair;
                stick_count = a;
                your_pick_sum = b;
            }
            None => break,
        }

        match handle_bot(&stick_count) {
            Some(new_stick_count) => {
                stick_count = new_stick_count;
            }
            None => break,
        }
        draw(&stick_count, &your_pick_sum);
    }
}

fn handle_user(stick_count: &i32, your_pick_sum: &i32) -> Option<(i32, i32)> {
    let mut pick = String::new();
    println!("{}", "1 or 2 ? : ".cyan());
    io::stdin()
        .read_line(&mut pick)
        .expect("Could not read the input");
    let mut pick: i32 = match pick.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };
    if !(pick >= 1 && pick <= 2) {
        if pick < 1 {
            pick = 1;
        } else if pick > 2 {
            pick = 2;
        }
    }
    let new_stick_count = stick_count - pick;
    let new_your_pick_sum = your_pick_sum + pick;
    if new_stick_count == 0 {
        println!("{}", "Bot won!".red());
        return Option::None;
    }
    return Option::Some((new_stick_count, new_your_pick_sum));
}

fn handle_bot(stick_count: &i32) -> Option<i32> {
    let bot_pick = match *stick_count {
        4 => 1,
        3 => 2,
        2 => 1,
        _ => rand::thread_rng().gen_range(1, 3), // 1 or 2,
    };

    println!("{} {}", "Bot chose".red(), bot_pick);
    let new_stick_count = *stick_count - bot_pick;

    if new_stick_count == 0 {
        println!("{}", "You won!".cyan());
        return Option::None;
    }
    return Option::Some(new_stick_count);
}

fn draw(stick_count: &i32, your_pick_sum: &i32) {
    let bot_pick_sum = 21 - stick_count - your_pick_sum;

    for _ in 1..=*your_pick_sum {
        print!("_");
    }

    for _ in 1..=*stick_count {
        print!("|");
    }

    for _ in 1..=bot_pick_sum {
        print!("_");
    }

    println!("\n==============================================");
}
