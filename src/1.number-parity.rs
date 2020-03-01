// Find the number parity
fn is_odd(num: u64) -> bool {
    // the last line of the curly brace enclosed block is what is returned from the block
    // blocks can evaluated to values
    // semicolon ends the line, and the last line should be without a semicolon to return a value
    num % 2 == 1;
}

// there are multiple types of data types
// u(8, 16, 32, 64) are for example unsigned/positive integers
// stack-allocated
fn display_parity(num: u64) {
    // immutable declaration
    let result = is_odd(num);
    // if statements are evaluated to values as well
    // this one is a good ol' ternary
    // println!, vec! and many other ones containing ! are called macros
    // much like C macros
    // println! gets replaced by a much bigger code before compilation
    println!("{} is {}", num, if result {"odd"} else {"even"});
}

fn main() {
    // normal for loop,
    // the =n shows inclusiveness
    for number in 0..=100 {
        display_parity(number)
    }
}
