use yew::{classes, function_component, html, Callback, Classes, Html, Properties};

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

    let css = classes!(
        "grid",
        "grid-cols-3",
        "grid-rows-3",
        "aspect-square",
        "border-white",
        "box-border",
        "p-3",
        tile_border_classes(props.index)
    );

    html! {
        <div class={css}>
            { children }
        </div>
    }
}

fn tile_border_classes(index: GridIndex) -> Classes {
    match index {
        GridIndex::UpperLeft => classes!("border-r", "border-b"),
        GridIndex::UpperRight => classes!("border-l", "border-b"),
        GridIndex::Up => classes!("border-l", "border-r", "border-b"),
        GridIndex::Left => classes!("border-t", "border-r", "border-b"),
        GridIndex::Right => classes!("border-l", "border-t", "border-b"),
        GridIndex::Down => classes!("border-l", "border-r", "border-t"),
        GridIndex::Center => classes!("border"),
        GridIndex::LowerLeft => classes!("border-t", "border-r"),
        GridIndex::LowerRight => classes!("border-t", "border-l"),
    }
}
