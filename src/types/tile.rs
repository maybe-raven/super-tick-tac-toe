use super::player::Player;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tile {
    #[default]
    Unmarked,
    Marked(Player),
}
