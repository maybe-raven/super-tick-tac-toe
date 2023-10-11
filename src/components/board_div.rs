use yew::{function_component, html, use_state_eq, Callback, Html, Properties};

use crate::{
    components::RegionDiv,
    types::{Board, GridIndex},
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

    let children: Vec<Html> = board
        .regions
        .iter()
        .enumerate()
        .map(|(index, &region)| {
            html! {
                <RegionDiv index={GridIndex::try_from(index).unwrap()} region={region} callback={callback.clone()} />
            }
        })
        .collect();

    html! {
        <div class="grid grid-cols-3 grid-rows-3 aspect-square mx-auto mt-12 max-w-lg">
            { children }
        </div>
    }
}
