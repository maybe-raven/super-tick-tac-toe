use serde::{Deserialize, Serialize};

use crate::{BoardItem, BoardOutcome, Player};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tile {
    #[default]
    Unmarked,
    Marked(Player),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkTileResult {
    /// No tile has been marked.
    NoChange,
    /// A tile has been marked, but the outcome has not been decided yet.
    TileMarked,
    /// A tile has been marked, and the outcome has been decided.
    OutcomeDecided(BoardOutcome),
}

impl BoardItem for Tile {
    fn is_markable(&self) -> bool {
        matches!(*self, Tile::Unmarked)
    }

    fn is_marked_by(&self, player: Player) -> bool {
        matches!(*self, Tile::Marked(p) if p == player)
    }
}
