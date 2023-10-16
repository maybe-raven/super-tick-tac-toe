use crate::{
    components::TileDiv,
    types::{BoardIndex, Region},
};
use yew::prelude::*;

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
            let onclick = if props.region.is_tile_enabled(tile_index) {
                props.callback.clone().map(|callback| {
                    let region_index = props.index;
                    let tile_index = tile_index;
                    Callback::from(move |_| callback.emit((region_index, tile_index)))
                })
            } else {
                None
            };

            html! {
                <TileDiv tile={tile} onclick={onclick} />
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
