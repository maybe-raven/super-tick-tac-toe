#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GameState {
    #[default]
    InProgress,
    Draw,
    CircleWins,
    SquareWins,
}
