// #![allow(non_snake_case)]
//
// use rusqlite::{Connection, Result};
// use serde_json;
// use std::path::PathBuf;

use ingest_spotify_data::build_database;

fn main() {
    build_database();
}

// fn main() {
//     let connection = Connection::open("/Users/alan/Desktop/_sqlite_db_test.db3");
//     match connection {
//         Ok(conn) => {
//             make_tables(&conn).expect("Could not make tables");
//             load_stream_history(&conn).expect("Could not load stream histor");
//         }
//         Err(_) => (),
//     };
//     println!("Hello, world!");
// }
