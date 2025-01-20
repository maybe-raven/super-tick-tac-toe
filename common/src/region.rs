use crate::{Board, BoardIndex, BoardItem, BoardOutcome, BoardState, MarkTileResult, Player, Tile};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Region {
    pub board: Board<Tile>,
    pub state: BoardState,
}

impl Region {
    /// Mark the specified tile with the specified player,
    /// then update the state of the region accordingly.
    pub fn mark_tile(&mut self, index: BoardIndex, player: Player) -> MarkTileResult {
        if !self.is_tile_enabled(index) {
            return MarkTileResult::NoChange;
        }

        self.board[index] = Tile::Marked(player);

        match self.board.get_state() {
            BoardState::InProgress => MarkTileResult::TileMarked,
            BoardState::Complete(outcome) => {
                self.state = BoardState::Complete(outcome);
                MarkTileResult::OutcomeDecided
            }
        }
    }

    pub fn is_tile_enabled(&self, index: BoardIndex) -> bool {
        self.is_markable() && self.board[index].is_markable()
    }
}

impl BoardItem for Region {
    fn is_marked_by(&self, player: Player) -> bool {
        matches!(self.state, BoardState::Complete(BoardOutcome::WonBy(p)) if p == player)
    }

    fn is_markable(&self) -> bool {
        matches!(self.state, BoardState::InProgress)
    }
}
