use crate::types::{Player, Tile};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub(crate) struct Props {
    pub(crate) tile: Tile,
    pub(crate) onclick: Option<Callback<MouseEvent>>,
}

#[function_component(TileDiv)]
pub(crate) fn tile_div(props: &Props) -> Html {
    let css = classes!(
        "aspect-square",
        "bg-gray-800",
        props.onclick.is_some().then_some("hover:bg-white"),
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

fn player_svg(player: Player) -> Html {
    html! {
        <svg class="w-full h-full" xmlns="http://www.w3.org/2000/svg">
            {
                match player {
                Player::Circle => html! {
                    <circle cx="50%" cy="50%" r="40%" stroke="white" stroke-width="3" fill="transparent" />
                },
                Player::Cross => html! {
                    <>
                        <line x1="10%" y1="10%" x2="90%" y2="90%" stroke="white" stroke-width="3" />
                        <line x1="90%" y1="10%" x2="10%" y2="90%" stroke="white" stroke-width="3" />
                    </>
                }
                }
            }
        </svg>
    }
}
