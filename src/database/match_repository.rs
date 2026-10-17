use super::{Player, RepoError, SqlitePlayerRepository};
use crate::{
    Game,
    core::{GameRepository, PlayerScore, Score},
};
use rusqlite::{Connection, OptionalExtension, Row, params};
use std::sync::{Arc, Mutex};

pub struct MatchParticipant {
    player_id: i64,
    seat_order: usize,
    score: Option<i16>,
    is_winner: bool,
}

impl MatchParticipant {
    pub fn new(player_id: i64, seat_order: usize, player_score: PlayerScore) -> Self {
        Self {
            player_id,
            seat_order,
            score: player_score.score,
            is_winner: player_score.is_winner,
        }
    }
}

pub struct NewMatch {
    game_id: String,
    state_json: Option<String>,
    score_json: Option<String>,
    participants: Vec<MatchParticipant>,
}

impl NewMatch {
    pub fn new(
        game_id: String,
        state_json: Option<String>,
        score_json: Option<String>,
        player_ids: &[i64],
        score: Score,
    ) -> Self {
        let participants = player_ids
            .iter()
            .zip(score.scores.iter())
            .enumerate()
            .map(|(seat_order, (&player_id, &player_score))| {
                MatchParticipant::new(player_id, seat_order, player_score)
            })
            .collect();
        Self {
            game_id,
            state_json,
            score_json,
            participants,
        }
    }
}

pub struct Match {
    id: i64,
    game_id: String,
    state_json: Option<String>,
    score_json: Option<String>,
    played_at: String,
}

pub struct MatchDetails {
    match_data: Match,
    game: Game,
    participants: Vec<(Player, MatchParticipant)>,
}

pub trait MatchRepository {
    fn create(&self, new_match: &NewMatch) -> Result<MatchDetails, RepoError>;
    fn get_by_id(&self, id: i64) -> Result<Option<MatchDetails>, RepoError>;
    fn list_by_game(&self, game_id: &str) -> Result<Vec<MatchDetails>, RepoError>;
    fn delete(&self, id: i64) -> Result<(), RepoError>;
}

pub struct SqliteMatchRepository {
    connection: Arc<Mutex<Connection>>,
    game_repository: GameRepository,
}

impl SqliteMatchRepository {
    pub fn new(connection: Arc<Mutex<Connection>>, game_repository: GameRepository) -> Self {
        Self {
            connection,
            game_repository,
        }
    }

    fn map_match_row(row: &Row) -> rusqlite::Result<Match> {
        Ok(Match {
            id: row.get(0)?,
            game_id: row.get(1)?,
            state_json: row.get(2)?,
            score_json: row.get(3)?,
            played_at: row.get(4)?,
        })
    }

    fn map_participant_row(row: &Row) -> rusqlite::Result<(Player, MatchParticipant)> {
        let player = SqlitePlayerRepository::map_player_row(row)?;

        let participant = MatchParticipant {
            player_id: row.get(0)?,
            seat_order: row.get(3)?,
            score: row.get(4)?,
            is_winner: row.get(5)?,
        };

        Ok((player, participant))
    }

    fn get_game(&self, game_id: &str) -> Result<Game, RepoError> {
        self.game_repository
            .get_by_id(game_id)
            .ok_or_else(|| RepoError::NotFound(format!("Game with id {game_id} not found")))
    }

    fn fetch_participants(
        connection: &Connection,
        match_id: i64,
    ) -> Result<Vec<(Player, MatchParticipant)>, RepoError> {
        let mut statement = connection.prepare(
            "SELECT p.id, p.name, p.created_at, mp.seat_order, mp.score, mp.is_winner
            FROM match_participants mp
            JOIN players p ON p.id = mp.player_id
            WHERE mp.match_id = ?1
            ORDER BY mp.seat_order ASC",
        )?;

        let participant_iterator =
            statement.query_map(params![match_id], Self::map_participant_row)?;

        let participants = participant_iterator.collect::<Result<Vec<_>, _>>()?;

        Ok(participants)
    }

    fn build_match_details(
        &self,
        connection: &Connection,
        match_data: Match,
    ) -> Result<MatchDetails, RepoError> {
        let game = self.get_game(&match_data.game_id)?;
        let participants = Self::fetch_participants(connection, match_data.id)?;

        Ok(MatchDetails {
            match_data,
            game,
            participants,
        })
    }
}

impl MatchRepository for SqliteMatchRepository {
    fn create(&self, new_match: &NewMatch) -> Result<MatchDetails, RepoError> {
        let game = self.get_game(&new_match.game_id)?;

        let mut connection = self.connection.lock().map_err(|_| RepoError::Lock)?;
        let tx = connection.transaction()?;

        let match_data = tx.query_row(
            "INSERT INTO matches (game_id, state_json, score_json)
             VALUES (?1, ?2, ?3)
             RETURNING id, game_id, state_json, score_json, played_at",
            params![
                new_match.game_id,
                new_match.state_json,
                new_match.score_json
            ],
            Self::map_match_row,
        )?;

        for participant in &new_match.participants {
            tx.execute(
                "INSERT INTO match_participants (match_id, player_id, seat_order, score, is_winner)
                VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    match_data.id,
                    participant.player_id,
                    participant.seat_order,
                    participant.score,
                    participant.is_winner
                ],
            )?;
        }

        let participants = Self::fetch_participants(&tx, match_data.id)?;

        tx.commit()?;

        Ok(MatchDetails {
            match_data,
            game,
            participants,
        })
    }

    fn get_by_id(&self, id: i64) -> Result<Option<MatchDetails>, RepoError> {
        let connection = self.connection.lock().map_err(|_| RepoError::Lock)?;

        let match_opt = connection
            .query_row(
                "SELECT id, game_id, state_json, score_json, played_at
                FROM matches
                WHERE id = ?1",
                params![id],
                Self::map_match_row,
            )
            .optional()?;

        match match_opt {
            Some(match_data) => Ok(Some(self.build_match_details(&connection, match_data)?)),
            None => Ok(None),
        }
    }

    fn list_by_game(&self, game_id: &str) -> Result<Vec<MatchDetails>, RepoError> {
        let game = self.get_game(game_id)?;
        let connection = self.connection.lock().map_err(|_| RepoError::Lock)?;

        let mut statement = connection.prepare(
            "SELECT id, game_id, state_json, score_json, played_at
            FROM matches
            WHERE game_id = ?1
            ORDER BY played_at DESC",
        )?;

        let match_iterator = statement.query_map(params![game_id], Self::map_match_row)?;
        let matches = match_iterator.collect::<Result<Vec<Match>, _>>()?;

        let mut details_list = Vec::with_capacity(matches.len());
        for match_data in matches {
            let participants = Self::fetch_participants(&connection, match_data.id)?;
            details_list.push(MatchDetails {
                match_data,
                game: game.clone(),
                participants,
            });
        }

        Ok(details_list)
    }

    fn delete(&self, id: i64) -> Result<(), RepoError> {
        let connection = self.connection.lock().map_err(|_| RepoError::Lock)?;

        let rows_affected = connection
            .execute("DELETE FROM matches WHERE id = ?1", params![id])
            .map_err(RepoError::Database)?;

        if rows_affected == 0 {
            Err(RepoError::NotFound(format!("Match with id {id} not found")))
        } else {
            Ok(())
        }
    }
}
