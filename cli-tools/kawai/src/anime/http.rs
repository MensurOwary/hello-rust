use serde_derive::{Deserialize, Serialize};
use crate::anime::Anime;
use crate::BASE_URL;
use crate::cli::CliArgs;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult {
    pub results: Vec<ApiResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "mal_id")]
    pub id: u32,
}

impl ApiResponse {
    pub fn load(&self) -> Result<Anime, Box<dyn std::error::Error>> {
        let url = format!("{}/anime/{}", BASE_URL, self.id);
        let resp: Anime = reqwest::blocking::get(&url)?
            .json::<Anime>()?;
        Ok(resp)
    }
}

pub fn send_request(args: &CliArgs) -> Result<ApiResult, Box<dyn std::error::Error>> {
    let genre = &args.genre;
    let limit = args.limit.unwrap();
    let anime_type = &args.anime_type;
    let query = match &args.query {
        Some(st) => String::from(st),
        None => String::from("&order_by=score&sort=desc")
    };

    let genres = genre.iter()
        .map(|gen| (*gen) as u32)
        .map(|val| format!("{}", val))
        .collect::<Vec<String>>().join(",");

    let anime_type = anime_type.iter()
        .map(|atype| String::from(atype.to_string()))
        .collect::<Vec<String>>().join(",");

    let url = format!("{}/search/anime?limit={}&type={}&genre={}&q={}",
                      BASE_URL,
                      limit,
                      anime_type,
                      genres,
                      query
    );
    let resp: ApiResult = reqwest::blocking::get(&url)?
        .json::<ApiResult>()?;
    Ok(resp)
}
