use std::collections::HashMap;
// Type inference lets us omit an explicit type signature (which
// would be `HashMap<String, String>` in this example).

fn main() {
    let eng_morse: HashMap<char, String> = get_map();

    let sentence = "this is a good sentence";

    let str: String = sentence.chars().into_iter()
        .filter_map(|x: char| eng_morse.get(&x))
        .map(|x| x.to_string())
        .collect::<Vec<String>>().join(" ");

    println!("{}", str);
}

fn get_map() -> HashMap<char, String>{
    let mut eng_morse = HashMap::new();
    eng_morse.insert('a', "._".to_string());
    eng_morse.insert('b', "_...".to_string());
    eng_morse.insert('c', "_._.".to_string());
    eng_morse.insert('d', "_..".to_string());
    eng_morse.insert('e', ".".to_string());
    eng_morse.insert('f', ".._.".to_string());
    eng_morse.insert('g', "__.".to_string());
    eng_morse.insert('h', "....".to_string());
    eng_morse.insert('i', "..".to_string());
    eng_morse.insert('j', ".___".to_string());
    eng_morse.insert('k', "_._".to_string());
    eng_morse.insert('l', "._..".to_string());
    eng_morse.insert('m', "__".to_string());
    eng_morse.insert('n', "_.".to_string());
    eng_morse.insert('o', "___".to_string());
    eng_morse.insert('p', ".__.".to_string());
    eng_morse.insert('q', "__._".to_string());
    eng_morse.insert('r', "._.".to_string());
    eng_morse.insert('s', "...".to_string());
    eng_morse.insert('t', "_".to_string());
    eng_morse.insert('u', ".._".to_string());
    eng_morse.insert('v', "..._".to_string());
    eng_morse.insert('w', ".__".to_string());
    eng_morse.insert('x', "_.._".to_string());
    eng_morse.insert('y', "_.__".to_string());
    eng_morse.insert('z', "__..".to_string());
    return eng_morse
}
