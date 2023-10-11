#[allow(dead_code)]
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
