#![allow(non_snake_case)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Track {
    pub endTime: String,
    pub artistName: String,
    pub trackName: String,
    pub msPlayed: u32,
}
