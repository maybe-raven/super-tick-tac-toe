use crate::Player;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoardState {
    #[default]
    InProgress,
    Complete(BoardOutcome),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoardOutcome {
    Draw,
    WonBy(Player),
}
