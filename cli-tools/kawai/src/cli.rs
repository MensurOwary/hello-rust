use structopt::StructOpt;
use crate::anime::structs::{AnimeGenre, AnimeType};

#[derive(Debug, StructOpt)]
pub struct CliArgs {
    #[structopt(short, long)]
    pub limit: Option<u32>,
    /// (multiple options / space separated) : movie, tv, ova
    #[structopt(short = "t", long = "type")]
    pub anime_type: Vec<AnimeType>,
    /// (multiple options / space separated) : action, adventure, cars, comedy, dementia,
    /// demons, mystery, drama, ecchi, fantasy, game, hentai,
    /// historical, horror, kids, magic, martialarts, mecha, music,
    /// parody, samurai, romance, school, scifi, shoujo, shoujoai,
    /// shounen, shounenai, space, sports, superpower, vampire,
    /// yaoi, yuri, harem, sliceoflife, supernatural, military, police,
    /// psychological, thriller, seinen, josei
    #[structopt(short, long)]
    pub genre: Vec<AnimeGenre>,
    #[structopt(short, long)]
    pub query: Option<String>,
}

impl CliArgs {
    pub fn initialize_defaults(self) -> CliArgs {
        let limit = match self.limit {
            Some(t) => Some(t),
            None => Some(5)
        };

        CliArgs {
            limit,
            ..self
        }
    }

}