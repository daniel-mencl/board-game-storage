use super::RepoError;

use std::sync::{Arc, Mutex};

use rusqlite::{Connection, OptionalExtension, Row, params};

pub struct Player {
    id: i64,
    name: String,
    created_at: String,
}

pub struct NewPlayer {
    name: String,
}

pub trait PlayerRepository {
    fn create(&self, new_player: &NewPlayer) -> Result<Player, RepoError>;
    fn get_by_id(&self, id: i64) -> Result<Option<Player>, RepoError>;
    fn get_by_name(&self, name: &str) -> Result<Option<Player>, RepoError>;
    fn list_all(&self) -> Result<Vec<Player>, RepoError>;
    fn delete(&self, id: i64) -> Result<(), RepoError>;
}

// ----------------------------------------------------------------------------

pub struct SqlitePlayerRepository {
    connection: Arc<Mutex<Connection>>,
}

impl SqlitePlayerRepository {
    fn map_row(row: &Row) -> rusqlite::Result<Player> {
        Ok(Player {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
        })
    }
}

impl PlayerRepository for SqlitePlayerRepository {
    fn create(&self, new_player: &NewPlayer) -> Result<Player, RepoError> {
        let connection = self.connection.lock().map_err(|_| RepoError::Lock)?;

        connection
            .query_row(
                "INSERT INTO players (name) VALUES (?1) RETURNING id, name, created_at",
                params![new_player.name],
                Self::map_row,
            )
            .map_err(|err| RepoError::Database(err))
    }

    fn get_by_id(&self, id: i64) -> Result<Option<Player>, RepoError> {
        let connection = self.connection.lock().map_err(|_| RepoError::Lock)?;

        connection
            .query_row(
                "SELECT id, name, created_at FROM players WHERE id = ?1",
                params![id],
                Self::map_row,
            )
            .optional()
            .map_err(RepoError::Database)
    }

    fn get_by_name(&self, name: &str) -> Result<Option<Player>, RepoError> {
        let connection = self.connection.lock().map_err(|_| RepoError::Lock)?;

        connection
            .query_row(
                "SELECT id, name, created_at FROM players WHERE name = ?1",
                params![name],
                Self::map_row,
            )
            .optional()
            .map_err(RepoError::Database)
    }

    fn list_all(&self) -> Result<Vec<Player>, RepoError> {
        let connection = self.connection.lock().map_err(|_| RepoError::Lock)?;

        let mut statement =
            connection.prepare("SELECT id, name, created_at FROM players ORDER BY name ASC")?;

        let player_iterator = statement.query_map([], Self::map_row)?;

        let players = player_iterator.collect::<Result<Vec<Player>, _>>()?;

        Ok(players)
    }

    fn delete(&self, id: i64) -> Result<(), RepoError> {
        let connection = self.connection.lock().map_err(|_| RepoError::Lock)?;

        let rows_affected = connection
            .execute("DELETE FROM players WHERE id = ?1", params![id])
            .map_err(RepoError::Database)?;

        if rows_affected == 0 {
            Err(RepoError::NotFound(format!(
                "Player with id {id} not found"
            )))
        } else {
            Ok(())
        }
    }
}
