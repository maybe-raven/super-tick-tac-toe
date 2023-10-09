use std::ops::{Index, IndexMut};

use super::{GameState, GridArray, GridIndex, Player, Region};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Board {
    pub(crate) regions: GridArray<Region>,
    pub(crate) state: GameState,
    pub(crate) current_player: Player,
}

impl Index<GridIndex> for Board {
    type Output = Region;

    fn index(&self, index: GridIndex) -> &Self::Output {
        self.regions.index(usize::from(index))
    }
}

impl IndexMut<GridIndex> for Board {
    fn index_mut(&mut self, index: GridIndex) -> &mut Self::Output {
        self.regions.index_mut(usize::from(index))
    }
}
