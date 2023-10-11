use super::Player;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GameState {
    #[default]
    InProgress,
    Draw,
    WonBy(Player),
}
