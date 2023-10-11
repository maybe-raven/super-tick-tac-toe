use yew::prelude::*;

use crate::{
    components::RegionDiv,
    types::{Board, GameState, GridIndex, Player},
    IsNoneOr,
};

#[derive(Clone, PartialEq, Eq, Properties)]
pub(crate) struct Props {
    pub(crate) board: Board,
}

#[function_component(BoardDiv)]
pub(crate) fn board_div() -> Html {
    let board = use_state_eq(Board::new);

    let callback = matches!(board.state, GameState::InProgress).then(|| {
        let state = board.clone();
        Callback::from(move |(region_index, tile_index): (GridIndex, GridIndex)| {
            let mut new_board = (*state).clone();
            new_board.mark_tile(region_index, tile_index);
            state.set(new_board);
        })
    });

    let target_region_index = board.target_region_index();

    let children: Vec<Html> = board
        .regions
        .iter()
        .enumerate()
        .map(|(index, &region)| {
            let grid_index = GridIndex::try_from(index).unwrap();

            let callback = (matches!(region.state, GameState::InProgress)
                && target_region_index.is_none_or(|target_index| target_index == grid_index))
            .then(|| callback.clone())
            .flatten();

            html! {
                <RegionDiv index={grid_index} region={region} callback={callback} />
            }
        })
        .collect();

    let current_player_text = match board.current_player {
        Player::Circle => "Current Player: Circle",
        Player::Cross => "Current Player: Cross",
    };

    let game_state_text = match board.state {
        GameState::InProgress => "Game In Progress".to_owned(),
        GameState::Draw => "Draw".to_owned(),
        GameState::WonBy(player) => {
            format!("Victor: {}", player)
        }
    };

    html! {
        <div class="flex flex-col mx-auto mt-12 max-w-lg text-center gap-1">
            <div class="grid grid-cols-3 grid-rows-3 aspect-square bg-white gap-0.5">
                { children }
            </div>
            <p>{ current_player_text }</p>
            <p>{ game_state_text }</p>
        </div>
    }
}
