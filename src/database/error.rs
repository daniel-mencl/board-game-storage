pub enum RepoError {
    Lock,
    Database(rusqlite::Error),
    NotFound(String),
}

impl From<rusqlite::Error> for RepoError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Database(err)
    }
}
