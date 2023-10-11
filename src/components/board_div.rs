use yew::{function_component, html, use_state_eq, Callback, Html, Properties};

use crate::{
    components::RegionDiv,
    types::{Board, GameState, GridIndex},
};

#[derive(Clone, PartialEq, Eq, Properties)]
pub(crate) struct Props {
    pub(crate) board: Board,
}

#[function_component(BoardDiv)]
pub(crate) fn board_div() -> Html {
    let board = use_state_eq(Board::new);

    let callback = {
        let state = board.clone();
        Callback::from(move |(region_index, tile_index): (GridIndex, GridIndex)| {
            let mut new_board = (*state).clone();
            new_board.mark_tile(region_index, tile_index);
            state.set(new_board);
        })
    };

    let enforce_target_region_index = board
        .target_region_index
        .is_some_and(|index| matches!(board[index].state, GameState::InProgress));

    let children: Vec<Html> = board
        .regions
        .iter()
        .enumerate()
        .map(|(index, &region)| {
            let grid_index = GridIndex::try_from(index).unwrap();

            let region_disabled = enforce_target_region_index && Some(grid_index) != board.target_region_index;

            html! {
                <RegionDiv index={grid_index} region={region} callback={callback.clone()} disabled={region_disabled} />
            }
        })
        .collect();

    html! {
        <div class="grid grid-cols-3 grid-rows-3 aspect-square mx-auto mt-12 max-w-lg">
            { children }
        </div>
    }
}
