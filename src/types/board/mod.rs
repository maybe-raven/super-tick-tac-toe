#[allow(clippy::module_inception)]
pub(crate) mod board;
pub(crate) mod board_enumerate;
pub(crate) mod board_index;
pub(crate) mod board_item;
pub(crate) mod board_state;

pub(crate) use {
    board::Board,
    board_enumerate::BoardEnumerate,
    board_index::BoardIndex,
    board_item::BoardItem,
    board_state::{BoardOutcome, BoardState},
};
