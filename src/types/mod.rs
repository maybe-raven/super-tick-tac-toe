pub(crate) mod board;
pub(crate) mod game;
pub(crate) mod player;
pub(crate) mod region;
pub(crate) mod tile;

pub(crate) use {
    board::{Board, BoardIndex, BoardOutcome, BoardState},
    game::Game,
    player::Player,
    region::Region,
    tile::{MarkTileResult, Tile},
};
