use crate::{ai, components::RegionDiv};
use common::{BoardIndex, BoardOutcome, BoardState, Game, MarkTileResult, Player};
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Properties)]
pub(crate) struct Props {
    pub(crate) use_ai: bool,
}

#[function_component(GameDiv)]
pub(crate) fn game_div(props: &Props) -> Html {
    let game = use_state_eq(Game::new);
    let use_ai = props.use_ai;
    let callback = {
        let state = game.clone();
        Callback::from(
            move |(region_index, tile_index): (BoardIndex, BoardIndex)| {
                let mut new_board = (*state).clone();
                match new_board.mark_tile(region_index, tile_index) {
                    MarkTileResult::NoChange => {
                        return;
                    }
                    MarkTileResult::TileMarked => {
                        if use_ai {
                            ai::random::make_move(&mut new_board);
                        }
                    }
                    MarkTileResult::OutcomeDecided => (),
                };
                state.set(new_board);
            },
        )
    };

    let children: Vec<Html> = game
        .board
        .enumerate()
        .map(|(index, &region)| {
            let callback = game.is_region_enabled(index).then(|| callback.clone());

            html! {
                <RegionDiv index={index} region={region} callback={callback} />
            }
        })
        .collect();

    let game_state_text = match game.state {
        BoardState::InProgress => match game.current_player {
            Player::Circle => "Current Player: Circle",
            Player::Cross => "Current Player: Cross",
        }
        .to_owned(),
        BoardState::Complete(BoardOutcome::Draw) => "Draw".to_owned(),
        BoardState::Complete(BoardOutcome::WonBy(player)) => {
            format!("Victor: {}", player)
        }
    };

    html! {
        <div class="flex flex-col mx-auto mt-12 max-w-lg text-center gap-1">
            <div class="grid grid-cols-3 grid-rows-3 aspect-square bg-white gap-0.5">
                { children }
            </div>
            <p>{ game_state_text }</p>
        </div>
    }
}
