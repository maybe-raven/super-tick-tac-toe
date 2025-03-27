pub(crate) mod game_div;
pub(crate) mod region_div;
pub(crate) mod tile_div;

use common::Player;
use yew::{html, Html};
pub(crate) use {
    game_div::{AIGameDiv, LMGameDiv},
    region_div::RegionDiv,
    tile_div::TileDiv,
};

fn player_svg(player: Player) -> Html {
    html! {
        <svg class="w-full h-full bg-fore" xmlns="http://www.w3.org/2000/svg">
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
