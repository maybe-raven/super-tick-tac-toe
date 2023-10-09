use yew::prelude::*;

use crate::{components::BoardDiv, types::Board};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BoardDiv board={Board::default()} />
    }
}
