use std::ops::{Index, IndexMut};

use super::{GameState, GridArray, GridIndex, Player, Region};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Board {
    pub(crate) regions: GridArray<Region>,
    pub(crate) state: GameState,
    pub(crate) current_player: Player,
}
impl Board {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn mark_tile(&mut self, region_index: GridIndex, tile_index: GridIndex) {
        // If the game is not in progress, then it's done, so we do nothing.
        if !matches!(self.state, GameState::InProgress) {
            return;
        }

        if self.regions[region_index].mark_tile(tile_index, self.current_player) {
            self.state = self.calc_state();
        }

        self.current_player = self.current_player.other();
    }

    fn calc_state(&self) -> GameState {
        if self.check_player(Player::Circle) {
            GameState::WonBy(Player::Circle)
        } else if self.check_player(Player::Cross) {
            GameState::WonBy(Player::Cross)
        } else if self.is_filled() {
            GameState::Draw
        } else {
            GameState::InProgress
        }
    }

    /// Check if the specified player has at least one three-in-a-line.
    fn check_player(&self, player: Player) -> bool {
        GridIndex::ALL_LINES.iter().any(|combo| {
            combo.iter().all(|&index| match self[index].state {
                GameState::WonBy(player_mark) => player_mark == player,
                _ => false,
            })
        })
    }

    /// Check if all the regions on the board have been completed.
    fn is_filled(&self) -> bool {
        !self
            .regions
            .iter()
            .any(|region| matches!(region.state, GameState::InProgress))
    }
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
