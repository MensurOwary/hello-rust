extern crate regex;

use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use regex::Regex;
use regex::RegexBuilder;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct CliArgs {
    /// The pattern to look for
    // short enables -p
    // long enables --pattern
    #[structopt(short, long)]
    pub pattern: String,
    /// The path to the file to read
    // PathBuf is like a String but for file system paths that works cross-platform.
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    pub path: std::path::PathBuf,
    /// If the search is case sensitive or not; default is true
    #[structopt(short = "c", long = "case-sensitive")]
    pub case_sensitive: Option<bool>,
}

impl CliArgs {
    fn initialize_default(self) -> CliArgs {
        let case_sensitive = match self.case_sensitive {
            Some(value) => Some(value),
            None => Some(true)
        };
        let pattern = self.pattern;
        let path = self.path;

        CliArgs {
            pattern,
            path,
            case_sensitive,
        }
    }
}

fn main() -> Result<(), ExitFailure> {
    // application arguments
    let args = CliArgs::from_args()
        .initialize_default(); // initialize the default values

    // read the content of the file to a string
    let content = std::fs::read_to_string(&args.path)
        // provide a context for a potential exception
        .with_context(|_| format!("Could not read file `{:?}`", &args.path))?;

    // regex template to find the searched word
    let regex_template = format!("(?P<word>{})", &args.pattern);

    // regex
    let regex: Regex = RegexBuilder::new(&regex_template)
        .case_insensitive(!args.case_sensitive.unwrap())
        .build().unwrap();

    // total occurrence count
    let mut total_occurrences = 0;

    // loop through the lines
    for (line_number, line) in content.lines().enumerate() {
        // get the occurrence in each line
        let occurrences = line.get_occurrences(&args.pattern, args.case_sensitive.unwrap());

        // if there's any occurrence
        if let Some(count) = occurrences {
            // increase the count
            total_occurrences += count;
            line.print_line(line_number, &regex);
        }
    }
    println!("---------------");
    println!("Total {} occurrences", total_occurrences);

    return Ok(());
}

trait MatchCount {
    fn get_occurrences(&self, pattern: &String, case_sensitive: bool) -> Option<usize>;
    fn print_line(&self, line_number: usize, regex: &Regex);
}

impl MatchCount for &str {
    fn get_occurrences(&self, pattern: &String, case_sensitive: bool) -> Option<usize> {
        let count = if case_sensitive {
            self.matches(pattern).count()
        } else {
            self
                .to_lowercase()
                .matches(pattern)
                .count()
        };
        if count == 0 {
            None
        } else {
            Some(count)
        }
    }

    fn print_line(&self, line_number: usize, regex: &Regex) {
        // color the words
        let replaced = regex.replace_all(self, "\u{001B}[31m$word\u{001B}[0m");
        // print it
        println!("{}.\t{}", line_number, replaced);
    }
}

