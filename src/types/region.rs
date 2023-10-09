use std::ops::{Index, IndexMut};

use super::{GameState, GridArray, GridIndex, Tile};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Region {
    pub(crate) tiles: GridArray<Tile>,
    pub(crate) state: GameState,
}

impl Index<GridIndex> for Region {
    type Output = Tile;

    fn index(&self, index: GridIndex) -> &Self::Output {
        self.tiles.index(usize::from(index))
    }
}

impl IndexMut<GridIndex> for Region {
    fn index_mut(&mut self, index: GridIndex) -> &mut Self::Output {
        self.tiles.index_mut(usize::from(index))
    }
}
