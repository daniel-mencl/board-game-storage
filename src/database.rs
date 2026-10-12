mod error;
mod match_repository;
mod player_repository;

use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

use error::RepoError;
use match_repository::{Match, MatchDetails, MatchParticipant, MatchRepository};
use player_repository::{NewPlayer, Player, PlayerRepository};
use rusqlite::Connection;

const SCHEMA_SQL: &str = include_str!("database/schema.sql");

pub fn establish_connection<P: AsRef<Path>>(
    path: P,
) -> Result<Arc<Mutex<Connection>>, Box<dyn Error>> {
    let conn = Connection::open(path)?;

    conn.pragma_update(None, "journal_mode", "WAL")?;

    conn.pragma_update(None, "foreign_keys", "ON")?;

    conn.execute_batch(SCHEMA_SQL)?;

    Ok(Arc::new(Mutex::new(conn)))
}
