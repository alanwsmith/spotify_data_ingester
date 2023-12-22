#![allow(non_snake_case)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Library {
    pub albums: Vec<LibraryAlbum>,
    pub artists: Vec<LibraryArtist>,
    pub bannedArtists: Vec<LibraryArtist>,
    pub bannedTracks: Vec<LibraryTrack>,
    pub episodes: Vec<LibraryEpisode>,
    pub other: Vec<String>,
    pub shows: Vec<LibraryShow>,
    pub tracks: Vec<LibraryTrack>,
}

#[derive(Deserialize, Debug)]
pub struct LibraryAlbum {
    pub artist: String,
    pub album: String,
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct LibraryArtist {
    pub name: String,
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct LibraryEpisode {}

#[derive(Deserialize, Debug)]
pub struct LibraryShow {}

#[derive(Deserialize, Debug)]
pub struct LibraryTrack {
    pub artist: String,
    pub album: String,
    pub track: String,
    pub uri: String,
}
