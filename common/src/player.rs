use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Player {
    #[default]
    Circle, // Always goes first.
    Cross,
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Player::Circle => Player::Cross,
            Player::Cross => Player::Circle,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::Circle => write!(f, "Circle"),
            Player::Cross => write!(f, "Cross"),
        }
    }
}
