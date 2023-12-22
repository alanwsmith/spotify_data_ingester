pub mod track;

use crate::track::Track;
use glob::glob;
use rusqlite::{Connection, Result};
use std::fs;
// use std::path::PathBuf;

pub fn build_database() {
    dbg!("building database");
    let connection = Connection::open("/Users/alan/Desktop/_sqlite_db_test.db3");
    match connection {
        Ok(conn) => {
            make_tables(&conn).expect("Could not make tables");
            load_stream_history(&conn).expect("Could not load stream histor");
        }
        Err(_) => (),
    };
}

fn load_stream_history(conn: &Connection) -> Result<()> {
    for entry in glob("data/StreamingHistor*.json").expect("Failed to read glob pattern") {
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
                    ).ok();
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
    conn.execute("DROP TABLE IF EXISTS marquee", ())?;
    conn.execute("DROP TABLE IF EXISTS stream_history", ())?;
    conn.execute(
        "CREATE TABLE library_tracks (artist TEXT, album TEXT, track TEXT, uri TEXT)",
        (),
    )?;
    conn.execute("CREATE TABLE marquee (artist TEXT, segment TEXT)", ())?;
    conn.execute(
        "CREATE TABLE stream_history (artist TEXT, track TEXT, end_time TEXT, ms_played INTEGER)",
        (),
    )?;
    Ok(())
}
