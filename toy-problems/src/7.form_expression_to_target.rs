/*
Write a program that outputs all possibilities
to put + or - or nothing between the numbers 1,2,…,9 (in this order)
such that the result is 100.
For example 1 + 2 + 3 - 4 + 5 + 6 + 78 + 9 = 100.

naive implementation
*/
use eval::eval;
use colored::*;

fn subroutine(tuple: (String, &[u8]), target: u8) {
    // expression
    let string  = tuple.0;
    // the rest of the array
    let arr = tuple.1;
    // if array has elements
    if arr.len() > 0 {
        // create those tuples again
        let first = (format!("{}+{}", string, arr[0]), &arr[1..]);
        let second = (format!("{}-{}", string, arr[0]), &arr[1..]);
        let third = (format!("{}{}", string, arr[0]), &arr[1..]);
        // pass to subroutines
        subroutine(first, target);
        subroutine(second, target);
        subroutine(third, target);
    } else {
        // if array is empty, it's time to stop and evaluate the
        // string we got so far
        let result = eval(&string).unwrap(); // eval librarry
        // if it is equal to the target
        if result == target {
            println!("{} = {}", string, result);
        }
    }
}
// target is the value we want to get
fn find_all_combinations(all: &Vec<u8>, target: u8) {
    // if there are more than one elements
    if all.len() > 1 {
        // do the +, - and nothing merging
        // form the expressions
        // and tuples like
        // (["1+2", [3...9]])
        // (["1-2", [3...9]])
        // (["12", [3...9]])
        let first = (format!("{}+{}", all[0], all[1]), &all[2..]);
        let second = (format!("{}-{}", all[0], all[1]), &all[2..]);
        let third = (format!("{}{}", all[0], all[1]), &all[2..]);
        // pass it to subroutines
        subroutine(first, target);
        subroutine(second, target);
        subroutine(third, target);
    } else if all.len() == 1 && all[0] == target{
        // in case of a single value
        println!("{} = {}", all[0], target);
    } else {
        // no elements case
        println!("{}", "Vector contains no elements".red()); // colors crate
    }
}

fn main() {
    let all = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    find_all_combinations(&all, 100);
}
