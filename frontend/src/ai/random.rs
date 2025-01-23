use rand::{rngs::ThreadRng, seq::IteratorRandom, thread_rng};

use common::{BoardIndex, BoardState, Game, MarkTileResult};

pub fn make_move(game: &mut Game) {
    let (region_index, tile_index) = thread_rng().generate_move(game);
    let result = game.mark_tile(region_index, tile_index);
    assert!(!matches!(result, MarkTileResult::NoChange));
}

pub(crate) trait GenerateMove {
    fn generate_move(&mut self, game: &Game) -> (BoardIndex, BoardIndex);
}

impl GenerateMove for ThreadRng {
    fn generate_move(&mut self, game: &Game) -> (BoardIndex, BoardIndex) {
        assert!(matches!(game.state, BoardState::InProgress));

        let region_index = game.allowed_region_index().unwrap_or_else(|| {
            game.board
                .unmarked()
                .choose(self)
                .expect("an in-progress game should always have at least incomplete region.")
                .0
        });
        let tile_index = game.board[region_index]
            .board
            .unmarked()
            .choose(self)
            .expect("an incomplete region should always have at least one unmarked tile.")
            .0;

        (region_index, tile_index)
    }
}
