use super::player::Player;

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tile {
    #[default]
    Unmarked,
    Marked(Player),
}
