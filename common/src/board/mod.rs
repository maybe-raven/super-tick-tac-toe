pub mod index;
pub mod item;
#[allow(clippy::module_inception)]
pub mod iter;
pub mod state;

use iter::Unmarked;
use serde::{Deserialize, Serialize};

pub use {
    index::BoardIndex,
    item::BoardItem,
    iter::BoardEnumerate,
    state::{BoardOutcome, BoardState},
};

use crate::Player;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Board<T> {
    pub tiles: [T; BoardIndex::N],
}

impl<T> Board<T> {
    pub fn enumerate(&self) -> BoardEnumerate<T> {
        BoardEnumerate::from(self)
    }
}

impl<T: BoardItem> Board<T> {
    pub fn unmarked(&self) -> Unmarked<T> {
        Unmarked::from(self)
    }

    /// Get the state of the board.
    pub fn get_state(&self) -> BoardState {
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
