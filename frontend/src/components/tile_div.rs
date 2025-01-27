use common::Tile;
use yew::prelude::*;

use crate::components::player_svg;

#[derive(Clone, PartialEq, Properties)]
pub(crate) struct Props {
    pub(crate) tile: Tile,
    pub(crate) onclick: Option<Callback<MouseEvent>>,
}

#[function_component(TileDiv)]
pub(crate) fn tile_div(props: &Props) -> Html {
    let enabled = props.onclick.is_some();
    let css = classes!(
        "aspect-square",
        if enabled { "bg-base" } else { "bg-fore" },
        enabled.then_some("hover:bg-white"),
    );

    html! {
        <div class={css} onclick={props.onclick.clone()}>
            {
                match props.tile {
                    Tile::Unmarked => html! {},
                    Tile::Marked(player) => html! {
                        player_svg(player)
                    }
                }
            }
        </div>
    }
}
