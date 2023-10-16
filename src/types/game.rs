use crate::IsNoneOr;

use super::{board::BoardItem, Board, BoardIndex, BoardState, MarkTileResult, Player, Region};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Game {
    pub(crate) board: Board<Region>,
    pub(crate) state: BoardState,
    pub(crate) current_player: Player,
    pub(crate) previous_play_index: Option<BoardIndex>,
}

impl Game {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Get the index of the region that the current player have to to play in.
    /// If it's `None`, that means the player can play in any region.
    pub(crate) fn allowed_region_index(&self) -> Option<BoardIndex> {
        let previous_play_index = self.previous_play_index?;
        self.board[previous_play_index]
            .is_markable()
            .then_some(previous_play_index)
    }

    pub(crate) fn is_region_enabled(&self, index: BoardIndex) -> bool {
        matches!(self.state, BoardState::InProgress)
            && self.allowed_region_index().is_none_or(|i| i == index)
    }

    pub(crate) fn mark_tile(
        &mut self,
        region_index: BoardIndex,
        tile_index: BoardIndex,
    ) -> MarkTileResult {
        if !self.is_region_enabled(region_index) {
            return MarkTileResult::NoChange;
        }

        let result = match self.board[region_index].mark_tile(tile_index, self.current_player) {
            MarkTileResult::NoChange => return MarkTileResult::NoChange,
            MarkTileResult::TileMarked => MarkTileResult::TileMarked,
            MarkTileResult::OutcomeDecided => match self.board.get_state() {
                BoardState::InProgress => MarkTileResult::TileMarked,
                BoardState::Complete(outcome) => {
                    self.state = BoardState::Complete(outcome);
                    MarkTileResult::OutcomeDecided
                }
            },
        };

        self.current_player = self.current_player.other();
        self.previous_play_index = Some(tile_index);

        result
    }
}
