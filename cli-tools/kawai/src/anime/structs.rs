use serde_derive::{Deserialize, Serialize};
use strum_macros::EnumString;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum AnimeType {
    Movie,
    TV,
    OVA,
}

impl AnimeType {
    pub fn to_string(&self) -> &str {
        match self {
            AnimeType::Movie => "movie",
            AnimeType::TV => "tv",
            AnimeType::OVA => "ova",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum AnimeGenre {
    Action = 1,
    Adventure = 2,
    Cars = 3,
    Comedy = 4,
    Dementia = 5,
    Demons = 6,
    Mystery = 7,
    Drama = 8,
    Ecchi = 9,
    Fantasy = 10,
    Game = 11,
    Hentai = 12,
    Historical = 13,
    Horror = 14,
    Kids = 15,
    Magic = 16,
    MartialArts = 17,
    Mecha = 18,
    Music = 19,
    Parody = 20,
    Samurai = 21,
    Romance = 22,
    School = 23,
    SciFi = 24,
    Shoujo = 25,
    ShoujoAi = 26,
    Shounen = 27,
    ShounenAi = 28,
    Space = 29,
    Sports = 30,
    SuperPower = 31,
    Vampire = 32,
    Yaoi = 33,
    Yuri = 34,
    Harem = 35,
    SliceOfLife = 36,
    Supernatural = 37,
    Military = 38,
    Police = 39,
    Psychological = 40,
    Thriller = 41,
    Seinen = 42,
    Josei = 43,
}