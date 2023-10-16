use yew::prelude::*;

use crate::{
    components::TileDiv,
    types::{board::BoardItem, BoardIndex, Region},
};

#[derive(Clone, PartialEq, Properties)]
pub(crate) struct Props {
    pub(crate) index: BoardIndex,
    pub(crate) region: Region,
    pub(crate) callback: Option<Callback<(BoardIndex, BoardIndex), ()>>,
}

#[function_component(RegionDiv)]
pub(crate) fn region_div(props: &Props) -> Html {
    let children: Html = props
        .region
        .board
        .enumerate()
        .map(|(tile_index, &tile)| {
            let onclick = if tile.is_markable() {
                props.callback.as_ref().map(|callback| {
                    let callback = callback.clone();
                    let region_index = props.index;
                    Callback::from(move |_| callback.emit((region_index, tile_index)))
                })
            } else {
                None
            };

            html! {
                <TileDiv index={tile_index} tile={tile} onclick={onclick} />
            }
        })
        .collect();

    html! {
        <div class="bg-gray-800 p-3">
            <div class="grid grid-cols-3 grid-rows-3 aspect-square gap-0.5 bg-white">
                { children }
            </div>
        </div>
    }
}
