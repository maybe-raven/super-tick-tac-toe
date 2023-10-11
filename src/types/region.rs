use std::ops::Index;

use super::{GameState, GridArray, GridIndex, Player, Tile};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Region {
    pub(crate) tiles: GridArray<Tile>,
    pub(crate) state: GameState,
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
    pub(crate) fn mark_tile(&mut self, index: GridIndex, player: Player) -> bool {
        if !matches!(self.state, GameState::InProgress) {
            return false;
        }

        let tile = &mut self.tiles[index];
        if matches!(*tile, Tile::Marked(_)) {
            return false;
        }
        *tile = Tile::Marked(player);

        let new_state = self.calc_state();
        if self.state != new_state {
            self.state = new_state;
            true
        } else {
            false
        }
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
            combo.iter().all(|&index| match self[index] {
                Tile::Marked(player_mark) => player_mark == player,
                _ => false,
            })
        })
    }

    /// Check if all the tiles in the region have been marked.
    fn is_filled(&self) -> bool {
        self.tiles
            .iter()
            .all(|&tile| matches!(tile, Tile::Marked(_)))
    }
}

impl Index<GridIndex> for Region {
    type Output = Tile;

    fn index(&self, index: GridIndex) -> &Self::Output {
        self.tiles.index(usize::from(index))
    }
}
