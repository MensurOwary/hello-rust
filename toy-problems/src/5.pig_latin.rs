/*
Write function that translates a text to Pig Latin and back. English is translated to Pig Latin by
taking the first letter of every word, moving it to the end of the word and adding ‘ay’.
“The quick brown fox” becomes “Hetay uickqay rownbay oxfay”.
*/
trait PigLatin {
    fn to_pig_latin(self) -> String;
}

trait Capitalize {
    fn capitalize(self) -> String;
}

impl PigLatin for &str {

    fn to_pig_latin(self) -> String {
        let len = self.len();
        let first = self.get(0..1).unwrap_or("").to_lowercase();
        let rest = self.get(1..len).unwrap_or("").to_lowercase();
        [rest, first, String::from("ay")].join("")
    }

}

impl Capitalize for &str {
    fn capitalize(self) -> String {
        let len = self.len();
        // Options can be 'unwrap'ped
        let first = self.get(0..1).unwrap_or("").to_uppercase();
        let rest = self.get(1..len).unwrap_or("").to_lowercase();
        // strings can be joined this way
        [first, rest].join("")
    }
}

fn main() {
    let sentence = "The quick brown fox";
    let result = sentence.split_whitespace()
        .map(|x: &str| x.to_pig_latin())
        .collect::<Vec<String>>().join(" ") // collectors
        .capitalize();
    println!("{}\n{}", sentence, result)
}
