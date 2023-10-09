use yew::{function_component, html, Html, Properties};

use crate::{
    components::RegionDiv,
    types::{Board, GridIndex},
};

#[derive(Clone, PartialEq, Eq, Properties)]
pub(crate) struct Props {
    pub(crate) board: Board,
}

#[function_component(BoardDiv)]
pub(crate) fn board_div(props: &Props) -> Html {
    let children: Vec<Html> = props
        .board
        .regions
        .iter()
        .enumerate()
        .map(|(index, &region)| {
            html! {
                <RegionDiv index={GridIndex::try_from(index).unwrap()} region={region} />
            }
        })
        .collect();

    html! {
        <div class="grid grid-cols-3 grid-rows-3 aspect-square mx-auto mt-12 max-w-lg">
            { children }
        </div>
    }
}
