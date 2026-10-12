use super::RepoError;

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
