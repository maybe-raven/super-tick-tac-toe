use super::{BoardEnumerate, BoardIndex, BoardItem, BoardOutcome, BoardState};
use crate::types::Player;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Board<T> {
    pub(crate) tiles: [T; BoardIndex::N],
}

impl<T> Board<T> {
    pub(crate) fn enumerate(&self) -> BoardEnumerate<T> {
        BoardEnumerate {
            iter: self.tiles.iter().enumerate(),
        }
    }
}

impl<T: BoardItem> Board<T> {
    /// Get the state of the board.
    pub(crate) fn get_state(&self) -> BoardState {
        if self.check_player(Player::Circle) {
            BoardState::Complete(BoardOutcome::WonBy(Player::Circle))
        } else if self.check_player(Player::Cross) {
            BoardState::Complete(BoardOutcome::WonBy(Player::Cross))
        } else if self.is_filled() {
            BoardState::Complete(BoardOutcome::Draw)
        } else {
            BoardState::InProgress
        }
    }

    /// Check if the specified player has at least one three-in-a-line.
    fn check_player(&self, player: Player) -> bool {
        BoardIndex::ALL_LINES.iter().any(|indices| {
            indices
                .iter()
                .all(|&index| self[index].is_marked_by(player))
        })
    }

    /// Check if all the tiles in the region have been marked.
    fn is_filled(&self) -> bool {
        !self.tiles.iter().any(|tile| tile.is_markable())
    }
}

impl<T> Index<BoardIndex> for Board<T> {
    type Output = T;

    fn index(&self, index: BoardIndex) -> &Self::Output {
        self.tiles.index(usize::from(index))
    }
}

impl<T> IndexMut<BoardIndex> for Board<T> {
    fn index_mut(&mut self, index: BoardIndex) -> &mut Self::Output {
        self.tiles.index_mut(usize::from(index))
    }
}
