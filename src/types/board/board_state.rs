use crate::types::Player;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BoardState {
    #[default]
    InProgress,
    Complete(BoardOutcome),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BoardOutcome {
    Draw,
    WonBy(Player),
}
