/*
Write a function that takes a list of strings an prints them, one per line,
in a rectangular frame.
For example the list ["Hello", "World", "in", "a", "frame"] gets printed as:

*********
* Hello *
* World *
* in    *
* a     *
* frame *
*********
*/
fn get_longest_length(collection: &Vec<&str>) -> usize {
    // functional programming, yay!
    return collection.iter()
            .map(|&x| x.len()) // lambda syntax
            .fold(0, |x, y| if x > y {x} else {y}) // reduce syntax // finding max
}

// easy stuff, _ represents 'ignore'
fn repeat_times(count: &usize, unit: &str) -> String {
    (0..*count).map(|_| unit).collect::<Vec<_>>().concat()
}

// Option represents optional value
// Some or None
fn print_line(input: Option<&str>, num: &usize) {
    // match is like Switch
    // should be exhaustible
    match input {
        Some(t) => {
            let remaining = num - t.len() - 3;
            let remaining = repeat_times(&remaining, " ");
            println!("* {}{}*", t, remaining);
        },
        None => println!("{}", repeat_times(num, "*"))
    }
}

fn display_frame(vector: Vec<&str>) {
    let width = get_longest_length(&vector) + 4; // 2 and 2 for padding on left/right
    // print the first line
    print_line(None, &width);
    for st in vector {
        print_line(Some(st), &width)
    }
    // print the last line
    print_line(None, &width);
}

fn main() {
    let vector = vec!["Hello", "World", "in", "a", "frame"];
    display_frame(vector);
}
