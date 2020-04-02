pub(crate) mod structs;
pub(crate) mod http;

use serde_derive::{Deserialize, Serialize};
use textwrap::fill;
use colored::*;
use crate::anime::structs::{AnimeType, Genre};

#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
    title: Option<String>,
    title_english: Option<String>,
    score: Option<f64>,
    rating: Option<String>,
    synopsis: Option<String>,
    #[serde(rename = "type")]
    anime_type: Option<AnimeType>,
    episodes: Option<u32>,
    duration: Option<String>,
    genres: Vec<Genre>,
}

impl Anime {
    fn title(&self) -> String {
        if let Some(title) = &self.title {
            title.replace(".", "").to_uppercase().bold().to_string()
        } else {
            String::from("<No Title>").red().bold().to_string()
        }
    }

    fn title_english(&self) -> String {
        if let Some(title_english) = &self.title_english {
            format!(" ( {} )", title_english.replace(".", "")).yellow().bold().to_string()
        } else {
            String::from(" ")
        }
    }

    fn score(&self) -> String {
        match &self.score {
            Some(score) => format!("{} : {}\n", "Score".bold(), score),
            None => format!("")
        }
    }

    fn rating(&self) -> String {
        format!("{} : {}\n", "Rated".bold(), get_or_default(&self.rating))
    }

    fn synopsis(&self) -> String {
        fill(&get_or_default(&self.synopsis), 80).replace("[Written by MAL Rewrite]", "")
    }

    fn anime_type(&self) -> String {
        match &self.anime_type {
            Some(t) => get_or_default(&Some(String::from(t.to_string()))),
            None => format!("")
        }
    }

    fn duration(&self) -> String {
        format!("{} : {}\n", "Running time".bold(), get_or_default(&self.duration))
    }

    fn episodes(&self) -> String {
        match self.anime_type {
            Some(AnimeType::Movie) => format!(""),
            _ => {
                match &self.episodes {
                    Some(episode) => {
                        let episodes_text = "Episodes".bold();
                        format!("{} : {}\n", episodes_text, episode)
                    }
                    None => format!("")
                }
            }
        }
    }

    fn genres(&self) -> String {
        let genres_text = "Genres".bold();
        let genres: String = self.genres.iter()
            .map(|genre| String::from(&genre.name).red().bold().to_string())
            .collect::<Vec<String>>()
            .join(" ");
        format!("{} : {}\n", genres_text, genres)
    }

    pub fn print(&self) {
        // header
        let rocket_emoji = "\u{1F680}";

        let anime_type = self.anime_type().cyan().bold();
        let title = self.title();
        let title_english = self.title_english();

        let header = format!("{}\t{} {} \t{}\n",
                             rocket_emoji,
                             title,
                             title_english,
                             anime_type,
        );

        // body
        let body = format!("\t{}{}{}{}{}\n{}",
                           self.score(),
                           &self.duration(),
                           &self.episodes(),
                           &self.rating(),
                           &self.genres(),
                           &self.synopsis());

        let body = body.replace("\n", "\n\t"); // tab shifts
        println!("{}{}", header, body);
    }
}

fn get_or_default(val: &Option<String>) -> String {
    if let Some(at) = val {
        String::from(at)
    } else {
        String::from("")
    }
}