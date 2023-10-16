use crate::types::BoardOutcome;

use super::{board::BoardItem, Board, BoardIndex, BoardState, MarkTileResult, Player, Tile};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Region {
    pub(crate) board: Board<Tile>,
    pub(crate) state: BoardState,
}

impl Region {
    /// Mark the specified tile with the specified player,
    /// then update the state of the region accordingly.
    /// Does nothing if the region game is already done.
    /// Does nothing if the tile is already marked.
    ///
    /// # Returns
    /// `true` if the state of the region has changed;
    /// `false` otherwise.
    pub(crate) fn mark_tile(&mut self, index: BoardIndex, player: Player) -> MarkTileResult {
        if !self.is_markable() {
            return MarkTileResult::NoChange;
        }

        let tile = &mut self.board[index];
        if !tile.is_markable() {
            return MarkTileResult::NoChange;
        }
        *tile = Tile::Marked(player);

        match self.board.get_state() {
            BoardState::InProgress => MarkTileResult::TileMarked,
            BoardState::Complete(outcome) => {
                self.state = BoardState::Complete(outcome);
                MarkTileResult::OutcomeDecided
            }
        }
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
