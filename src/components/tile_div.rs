use yew::{classes, function_component, html, Classes, Html, Properties};

use crate::types::{GridIndex, Player, Tile};

#[derive(Clone, PartialEq, Eq, Properties)]
pub(crate) struct Props {
    pub(crate) index: GridIndex,
    pub(crate) tile: Tile,
}

impl Props {
    fn is_interactive(&self) -> bool {
        matches!(self.tile, Tile::Unmarked)
    }
}

#[function_component(TileDiv)]
pub(crate) fn tile_div(props: &Props) -> Html {
    let css = classes!(
        "aspect-square",
        "border-white",
        "box-border",
        props.is_interactive().then_some("hover:bg-white"),
        tile_border_classes(props.index)
    );

    html! {
        <div class={css}>
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
