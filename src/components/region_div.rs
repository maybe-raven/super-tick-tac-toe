use yew::{function_component, html, Callback, Html, Properties};

use crate::{
    components::TileDiv,
    types::{GameState, GridIndex, Region},
};

#[derive(Clone, PartialEq, Properties)]
pub(crate) struct Props {
    pub(crate) index: GridIndex,
    pub(crate) region: Region,
    pub(crate) callback: Callback<(GridIndex, GridIndex), ()>,
    pub(crate) disabled: bool,
}

#[function_component(RegionDiv)]
pub(crate) fn region_div(props: &Props) -> Html {
    let tiles_disabled = props.disabled || !matches!(props.region.state, GameState::InProgress);

    let children: Html = props
        .region
        .tiles
        .iter()
        .enumerate()
        .map(|(index, &tile)| {
            let tile_index = GridIndex::try_from(index).unwrap();

            let onclick = {
                let callback = props.callback.clone();
                let region_index = props.index;
                Callback::from(move |_| callback.emit((region_index, tile_index)))
            };

            html! {
                <TileDiv index={tile_index} tile={tile} onclick={onclick} disabled={tiles_disabled} />
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
