use serde::{Deserialize, Serialize};

use crate::Player;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoardState {
    #[default]
    InProgress,
    Complete(BoardOutcome),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoardOutcome {
    Draw,
    WonBy(Player),
}
