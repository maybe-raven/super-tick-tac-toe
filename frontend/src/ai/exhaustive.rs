use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
};

use common::{BoardItem, BoardOutcome, BoardState, Game, MarkTileResult, Player};
use gloo_console::log;

pub fn make_move(game: &mut Game) {
    if !matches!(game.state, BoardState::InProgress) {
        return;
    }

    let region_indices = if let Some(index) = game.allowed_region_index() {
        vec![(index, &game.board[index])]
    } else {
        game.board
            .enumerate()
            .filter(|(_, region)| region.is_markable())
            .collect()
    };
    let mut all_moves = Vec::new();
    let mut cache = HashMap::new();
    for (region_index, region) in region_indices {
        for (tile_index, _) in region
            .board
            .enumerate()
            .filter(|(_, tile)| tile.is_markable())
        {
            let mut game_clone = game.clone();
            game_clone.mark_tile(region_index, tile_index);
            let score = simulate(&game_clone, &mut cache).win_ratio();
            all_moves.push((score, region_index, tile_index));
            log!(format!(
                "score: {score}; indices: {:?}",
                (region_index, tile_index)
            ));
        }
    }
    let (_, region_index, tile_index) = all_moves
        .into_iter()
        .max_by(|(score_a, _, _), (score_b, _, _)| score_a.total_cmp(score_b))
        .expect("should have at least one possible move.");
    assert!(!matches!(
        game.mark_tile(region_index, tile_index),
        MarkTileResult::NoChange
    ));
}

#[derive(Clone, Copy, Default)]
struct Score {
    wins: usize,
    nonwins: usize,
}

impl Score {
    fn win_ratio(&self) -> f32 {
        self.wins as f32 / self.nonwins as f32
    }
}

impl Add for Score {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Score {
            wins: self.wins + rhs.wins,
            nonwins: self.nonwins + rhs.nonwins,
        }
    }
}
impl AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        self.wins += rhs.wins;
        self.nonwins += rhs.nonwins;
    }
}

fn simulate(game: &Game, cache: &mut HashMap<Game, Score>) -> Score {
    assert!(matches!(game.state, BoardState::InProgress));

    if let Some(&score) = cache.get(game) {
        return score;
    }

    let mut score = Score::default();
    for (region_index, region) in game
        .board
        .enumerate()
        .filter(|(index, region)| game.is_region_enabled(*index) && region.is_markable())
    {
        for (tile_index, _) in region
            .board
            .enumerate()
            .filter(|(_, tile)| tile.is_markable())
        {
            let mut game_clone = game.clone();
            match game_clone.mark_tile(region_index, tile_index) {
                MarkTileResult::NoChange => panic!("only markable indices should be used and this should never results in NoChange."),
                MarkTileResult::TileMarked => {
                    let sub_score = simulate(&game_clone, cache);
                    cache.insert(game_clone, sub_score);
                    score += sub_score;
                },
                MarkTileResult::OutcomeDecided(outcome) => match outcome {
                    BoardOutcome::Draw |
                    BoardOutcome::WonBy(Player::Circle) => score.nonwins += 1,
                    BoardOutcome::WonBy(Player::Cross) => score.wins += 1,
                }
            }
        }
    }

    score
}
