pub enum RepoError {
    Database(rusqlite::Error),
    GameNotFound(String),
    PlayerNotFound(i64),
    InvalidState(String),
}

impl From<rusqlite::Error> for RepoError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Database(err)
    }
}
