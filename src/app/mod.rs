use crate::components::BoardDiv;
use yew::prelude::*;
use yew_router::prelude::*;

use self::{home::Home, how_to_play::HowToPlay};

mod home;
mod how_to_play;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/howto")]
    HowToPlay,
    #[at("/local")]
    LocalGame,
    #[at("/create")]
    CreateOnlineGame,
    #[at("/join/:id")]
    JoinOnlineGame,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::HowToPlay => html! { <HowToPlay /> },
        Route::LocalGame => html! { <BoardDiv /> },
        Route::CreateOnlineGame => html! { <BoardDiv /> },
        Route::JoinOnlineGame { .. } => html! { <BoardDiv /> },
        Route::NotFound => html! { <BoardDiv /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
