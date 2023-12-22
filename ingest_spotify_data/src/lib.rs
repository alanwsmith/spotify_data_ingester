pub mod spotify_library;
pub mod track;

use crate::spotify_library::SpotifyLibrary;
use crate::track::Track;
use glob::glob;
use rusqlite::{Connection, Result};
use std::fs;

pub fn build_database() {
    dbg!("building database");
    let connection = Connection::open("spotify_data.sqlite3");
    match connection {
        Ok(conn) => {
            make_tables(&conn).expect("Could not make tables");
            load_stream_history(&conn).expect("Could not load stream history");
            load_library_tracks(&conn).expect("Could not load library tracks");
        }
        Err(_) => (),
    };
}

fn load_library_tracks(conn: &Connection) -> Result<()> {
    for entry in glob("YourLibrary.json").expect("Failed to read YourLibrary glob pattern") {
        match entry {
            Ok(path) => {
                dbg!(&path);
                match fs::read_to_string(path) {
                    Ok(raw) => match serde_json::from_str::<SpotifyLibrary>(raw.as_str()) {
                        Ok(data) => {
                            data.tracks.into_iter().for_each(|t| {
                                let track_id = &t.uri.split(":").collect::<Vec<&str>>()[2];
                                conn.execute(
                                    "INSERT INTO library_tracks (artist, album, track, uri, id) VALUES (?1, ?2, ?3, ?4, ?5)",
                                    (t.artist, t.album, t.track, &t.uri, track_id),
                                ).expect("Could not insert library track row");
                                ()
                            });
                        }
                        Err(e) => println!("{:?}", e),
                    },
                    Err(e) => println!("{:?}", e),
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}

fn load_stream_history(conn: &Connection) -> Result<()> {
    for entry in
        glob("StreamingHistor*.json").expect("Failed to read StreamingHistory glob pattern")
    {
        match entry {
            Ok(path) => {
                dbg!(&path);
                match fs::read_to_string(path) {
                    Ok(raw) => match serde_json::from_str::<Vec<Track>>(raw.as_str()) {
                        Ok(data) => {
                            data.into_iter().for_each(|d| {
                                conn.execute(
                                    "INSERT INTO stream_history (artist, track, end_time, ms_played) VALUES (?1, ?2, ?3, ?4)",
                                    (d.artistName, d.trackName, d.endTime, d.msPlayed),
                                ).expect("Could not insert stream history row");
                                ()
                            });
                        }
                        Err(e) => println!("{:?}", e),
                    },
                    Err(e) => println!("{:?}", e),
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}

fn make_tables(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE IF EXISTS library_tracks", ())?;
    conn.execute("DROP TABLE IF EXISTS stream_history", ())?;
    conn.execute(
        "CREATE TABLE library_tracks (artist TEXT, album TEXT, track TEXT, uri TEXT, id TEXT)",
        (),
    )?;
    conn.execute(
        "CREATE TABLE stream_history (artist TEXT, track TEXT, end_time TEXT, ms_played INTEGER)",
        (),
    )?;
    Ok(())
}
