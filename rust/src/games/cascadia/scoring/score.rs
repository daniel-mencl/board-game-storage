use super::{CascadiaAnimalScore, CascadiaBoard, CascadiaHabitat, CascadiaState};
use crate::{
    boards::{HexBoard, HexCoordinates}, core::{PlayerScore, Score, ScoreInto},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaScoreBreakdown {
    players: Vec<CascadiaPlayerScore>,
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub struct CascadiaPlayerScore {
    animals: CascadiaAnimalScore,
    habitats: CascadiaHabitatScore,
    nature_tokens: u16,
    total: u16
}
impl CascadiaPlayerScore {
    fn new(
        animals: CascadiaAnimalScore,
        habitats: CascadiaHabitatScore,
        nature_tokens: u16,
    ) -> CascadiaPlayerScore {
        let total = animals.total() + habitats.total() + nature_tokens;
        CascadiaPlayerScore {
            animals,
            habitats,
            nature_tokens,
            total
        }
    }

    pub fn total(&self) -> u16 {
        self.total
    }

    pub fn nature_tokens(&self) -> u16 {
        self.nature_tokens
    }
}

pub struct CascadiaRawHabitatScore {
    rivers: u16,
    wetlands: u16,
    forests: u16,
    prairies: u16,
    mountains: u16,
}
impl CascadiaRawHabitatScore {
    fn new(
        rivers: u16,
        wetlands: u16,
        forests: u16,
        prairies: u16,
        mountains: u16,
    ) -> CascadiaRawHabitatScore {
        CascadiaRawHabitatScore {
            rivers,
            wetlands,
            forests,
            prairies,
            mountains,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub struct CascadiaSoloHabitatScore {
    size: u16,
    bonus: u16,
}
impl CascadiaSoloHabitatScore {
    pub fn total(&self) -> u16 {
        self.size + self.bonus
    }
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub struct CascadiaHabitatScore {
    rivers: CascadiaSoloHabitatScore,
    wetlands: CascadiaSoloHabitatScore,
    forests: CascadiaSoloHabitatScore,
    prairies: CascadiaSoloHabitatScore,
    mountains: CascadiaSoloHabitatScore,
    total: u16,
}
impl CascadiaHabitatScore {
    pub fn new(
        rivers: CascadiaSoloHabitatScore,
        wetlands: CascadiaSoloHabitatScore,
        forests: CascadiaSoloHabitatScore,
        prairies: CascadiaSoloHabitatScore,
        mountains: CascadiaSoloHabitatScore,
    ) -> CascadiaHabitatScore {
        let total = rivers.total()
            + wetlands.total()
            + forests.total()
            + prairies.total()
            + mountains.total();

        CascadiaHabitatScore {
            rivers,
            wetlands,
            forests,
            prairies,
            mountains,
            total,
        }
    }

    pub fn total(&self) -> u16 {
        self.total
    }
}

fn opposite_index(index: usize) -> usize {
    if index >= 3 { index - 3 } else { index + 3 }
}

fn habitat_neighbors(
    board: &HexBoard<[CascadiaHabitat; 6]>,
    coord: HexCoordinates,
    habitats: &[CascadiaHabitat; 6],
    habitat: CascadiaHabitat,
) -> Vec<HexCoordinates> {
    let mut result = Vec::new();

    for (index, &direction) in HexCoordinates::CLOCKWISE_OFFSETS.iter().enumerate() {
        if habitats[index] != habitat {
            continue;
        }

        if let Some(other) = board.as_ref().get(&(coord + direction)) {
            if other[opposite_index(index)] == habitat {
                result.push(coord + direction);
            }
        }
    }

    result
}

fn score_habitat(board: &HexBoard<[CascadiaHabitat; 6]>, habitat: CascadiaHabitat) -> u16 {
    board
        .to_unweighted_graph(
            |_, tile| tile.contains(&habitat),
            |coord, tile| habitat_neighbors(board, coord, tile, habitat),
        )
        .components()
        .into_iter()
        .map(|component| component.len())
        .max()
        .unwrap_or(0) as u16
}

fn score_habitats(board: &CascadiaBoard) -> CascadiaRawHabitatScore {
    let habitats = board.habitats();
    let rivers = score_habitat(&habitats, CascadiaHabitat::River);
    let wetlands = score_habitat(&habitats, CascadiaHabitat::Wetland);
    let forests = score_habitat(&habitats, CascadiaHabitat::Forest);
    let prairies = score_habitat(&habitats, CascadiaHabitat::Prairie);
    let mountains = score_habitat(&habitats, CascadiaHabitat::Mountain);
    CascadiaRawHabitatScore::new(rivers, wetlands, forests, prairies, mountains)
}

fn calculate_bonus_values(players: usize, highest: usize, second_highest: usize) -> (u16, u16) {
    if players > 2 {
        if highest > 1 {
            (4 / highest as u16, 0)
        } else {
            if second_highest > 1 { (3, 0) } else { (3, 1) }
        }
    } else {
        if highest > 1 { (1, 0) } else { (2, 0) }
    }
}

fn calculate_habitat_bonus(sizes: Vec<u16>) -> Vec<CascadiaSoloHabitatScore> {
    let mut sorted = sizes.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    sorted.dedup();
    let highest = sorted.get(0).expect("At least one unique size");
    let second_highest = sorted.get(1);

    let highest_count = sizes.iter().filter(|x| *x == highest).count();
    let second_highest_count = match second_highest {
        Some(value) => sizes.iter().filter(|x| *x == value).count(),
        None => 0,
    };

    let (highest_bonus, second_highest_bonus) =
        calculate_bonus_values(sizes.len(), highest_count, second_highest_count);

    sizes
        .into_iter()
        .map(|size| CascadiaSoloHabitatScore {
            size,
            bonus: if size == *highest {
                highest_bonus
            } else if let Some(value) = second_highest {
                if size == *value {
                    second_highest_bonus
                } else {
                    0
                }
            } else {
                0
            },
        })
        .collect()
}

fn calculate_habitat_bonuses(habitats: Vec<CascadiaRawHabitatScore>) -> Vec<CascadiaHabitatScore> {
    let rivers: Vec<_> = habitats.iter().map(|x| x.rivers).collect();
    let rivers = calculate_habitat_bonus(rivers);

    let wetlands: Vec<_> = habitats.iter().map(|x| x.wetlands).collect();
    let wetlands = calculate_habitat_bonus(wetlands);

    let forests: Vec<_> = habitats.iter().map(|x| x.forests).collect();
    let forests = calculate_habitat_bonus(forests);

    let prairies: Vec<_> = habitats.iter().map(|x| x.prairies).collect();
    let prairies = calculate_habitat_bonus(prairies);

    let mountains: Vec<_> = habitats.iter().map(|x| x.mountains).collect();
    let mountains = calculate_habitat_bonus(mountains);

    (0..habitats.len())
        .into_iter()
        .map(|index| {
            CascadiaHabitatScore::new(
                rivers[index],
                wetlands[index],
                forests[index],
                prairies[index],
                mountains[index],
            )
        })
        .collect()
}

impl ScoreInto<Score> for CascadiaScoreBreakdown {
    fn score(&self) -> Score {
        let max_score = self.players.iter().map(|player| player.total()).max().expect("More than zero players");
        let max_score_max_nature = self.players.iter().filter(|player| player.total() == max_score).map(|player| player.nature_tokens()).max().expect("At least one player has max score");
        let player_scores: Vec<_> = self.players.iter().map(|player| PlayerScore::new(player.total() as i16, player.total() == max_score && player.nature_tokens() == max_score_max_nature)).collect();
        Score::new(player_scores)
    }
}

impl ScoreInto<CascadiaScoreBreakdown> for CascadiaState {
    fn score(&self) -> CascadiaScoreBreakdown {
        let animals: Vec<_> = self
            .players
            .iter()
            .map(|board| self.scoring_cards.score(board))
            .collect();

        let habitats: Vec<_> = self
            .players
            .iter()
            .map(|board| score_habitats(board))
            .collect();

        let habitats = calculate_habitat_bonuses(habitats);

        let nature_tokens: Vec<_> = self
            .players
            .iter()
            .map(|board| board.nature_tokens())
            .collect();

        let players = (0..self.players.len())
            .map(|index| {
                CascadiaPlayerScore::new(animals[index], habitats[index], nature_tokens[index])
            })
            .collect();

        CascadiaScoreBreakdown { players }
    }
}

pub fn map_count_to_points(count: usize, lowest: usize, points: &[u16], zero_past: bool) -> u16 {
    // index 0 -> points for count = 0 + lowest, index 1 -> points for count = 1 + lowest
    // past the vec end -> 0 if zero_past else max
    if count < lowest {
        0
    } else if count - lowest < points.len() {
        points[count - lowest]
    } else if zero_past {
        0
    } else {
        *points.last().unwrap()
    }
}
