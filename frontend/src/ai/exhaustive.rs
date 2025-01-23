use std::collections::HashMap;

use common::{BoardItem, BoardOutcome, BoardState, Game, MarkTileResult, Player};

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
            let score = simulate(&game_clone, &mut cache);
            all_moves.push((score, region_index, tile_index));
        }
    }
    let (_, region_index, tile_index) = all_moves
        .into_iter()
        .max_by_key(|&(score, _, _)| score)
        .expect("should have at least one possible move.");
    assert!(!matches!(
        game.mark_tile(region_index, tile_index),
        MarkTileResult::NoChange
    ));
}

fn simulate(game: &Game, cache: &mut HashMap<Game, usize>) -> usize {
    assert!(matches!(game.state, BoardState::InProgress));

    if let Some(&score) = cache.get(game) {
        return score;
    }

    let mut score = 0;
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
                    BoardOutcome::Draw => (),
                    BoardOutcome::WonBy(Player::Circle) => score -= 1,
                    BoardOutcome::WonBy(Player::Cross) => score += 1,
                }
            }
        }
    }

    score
}
