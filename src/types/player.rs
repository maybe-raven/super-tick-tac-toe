use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Player {
    #[default]
    Circle, // Always goes first.
    Cross,
}

impl Player {
    pub(crate) fn other(self) -> Self {
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
