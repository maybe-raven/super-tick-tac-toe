use rand::{distributions::Uniform, thread_rng, Rng};

use common::{BoardIndex, BoardItem, Game, MarkTileResult};
use tracing::{debug, debug_span, instrument};

#[instrument]
pub fn make_move(game: &mut Game) {
    let mut rng = thread_rng();
    let distr = Uniform::new(0, 9);
    let mut gen_index = || {
        BoardIndex::try_from(rng.sample(distr))
            .expect("rng range should be capped to the number of valid indices")
    };
    let region_index = game.allowed_region_index().unwrap_or_else(|| {
        let span = debug_span!("generate_region_index");
        let _enter = span.enter();
        loop {
            let i = gen_index();
            debug!(index=?i);
            if game.board[i].is_markable() {
                return i;
            }
        }
    });
    let span = debug_span!("attemping_random_moves", region_index=?region_index);
    let _enter = span.enter();
    loop {
        let tile_index = gen_index();
        debug!(tile_index=?tile_index);
        if !matches!(
            game.mark_tile(region_index, tile_index),
            MarkTileResult::NoChange
        ) {
            return;
        }
    }
}
