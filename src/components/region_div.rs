use yew::{classes, function_component, html, Classes, Html, Properties};

use crate::{
    components::TileDiv,
    types::{GridIndex, Region},
};

#[derive(Clone, PartialEq, Eq, Properties)]
pub(crate) struct Props {
    pub(crate) index: GridIndex,
    pub(crate) region: Region,
}

#[function_component(RegionDiv)]
pub(crate) fn region_div(props: &Props) -> Html {
    let children: Html = props
        .region
        .tiles
        .iter()
        .enumerate()
        .map(|(index, &tile)| {
            html! {
                <TileDiv index={GridIndex::try_from(index).unwrap()} tile={tile} />
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
