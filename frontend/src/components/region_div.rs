use crate::components::{player_svg, TileDiv};
use common::{BoardIndex, BoardItem, BoardState, Region};
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

    let inner =
        if let BoardState::Complete(common::BoardOutcome::WonBy(winner)) = props.region.state {
            player_svg(winner)
        } else {
            html! {
                <div class="grid grid-cols-3 grid-rows-3 aspect-square gap-0.5 bg-white">
                    { children }
                </div>
            }
        };

    let css = classes!(
        "p-3",
        if props.callback.is_some() && props.region.is_markable() {
            "bg-gray-800"
        } else {
            "bg-neutral-600"
        }
    );

    html! {
        <div class={css}>
            { inner }
        </div>
    }
}
