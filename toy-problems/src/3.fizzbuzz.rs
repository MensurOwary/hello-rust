// a naive FizzBuzz example
fn fizz_buzz() {
    // mutable integer
    // we can mutate the value in the further calls
    let mut curr = 1;
    // one of three loop structures (loop, while, for)
    // infinite loop, by default
    loop {
        // evaluating a big if/else if/else statement into a value
        let result = if curr % 15 == 0 {
            // doing this because the last one int.to_string() method
            // returns a String, not a primitive one,
            // and all the return types of the if blocks must be consistent
            String::from("FizzBuzz")
        } else if curr % 5 == 0 {
            String::from("Buzz")
        } else if curr % 3 == 0 {
            String::from("Fizz")
        } else {
            curr.to_string()
        };
        println!("{}", result);
        curr = curr + 1;
        // break part
        if curr == 151 {
            break;
        }
    }
}

fn main() {
    fizz_buzz()
}
