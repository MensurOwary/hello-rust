mod anime;
mod cli;

use dialoguer::Confirmation;
use std::io::{Write, stdout};
use termion::raw::IntoRawMode;
use structopt::StructOpt;

use crate::anime::http::{send_request};
use crate::cli::CliArgs;

const NEXT_ONE_TEMPLATE: &str = "Load the next one ";
const BASE_URL: &str = "https://api.jikan.moe/v3";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::from_args().initialize_defaults();
    let request = send_request(&args);

    if let Ok(response) = request {
        let mut size = response.results.len();

        for anime in response.results {
            size -= 1;
            let result = anime.load();
            if let Ok(anime_result) = result {
                anime_result.print();
                if size == 0 || !ask_confirmation() {
                    break;
                }
            } else if let Err(err) = result {
                println!("Failed to get {}", anime.id);
                println!("Cause : {:?}", err);
            }
        }
    } else {
        println!("Unsuccessful request");
    }

    Ok(())
}

fn ask_confirmation() -> bool {
    Confirmation::new().with_text(&NEXT_ONE_TEMPLATE).interact().unwrap_or(false)
}

#[allow(dead_code)]
fn flush(inp: &str, left_amount: u16) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let _ = writeln!(stdout, "{}{}{}{:>80}{}",
                     termion::cursor::Up(1),
                     termion::cursor::Left(left_amount),
                     inp,
                     "",
                     termion::cursor::Left(100));
}
