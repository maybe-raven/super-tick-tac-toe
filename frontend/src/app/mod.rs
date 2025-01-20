use self::{home::Home, how_to_play::HowToPlay};
use crate::components::GameDiv;
use yew::prelude::*;
use yew_router::prelude::*;

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
    #[at("/ai")]
    AiGame,
    // #[at("/create")]
    // CreateOnlineGame,
    // #[at("/join/:id")]
    // JoinOnlineGame,
    // #[not_found]
    // #[at("/404")]
    // NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::HowToPlay => html! { <HowToPlay /> },
        Route::LocalGame => html! { <GameDiv use_ai={false} /> },
        Route::AiGame => html! { <GameDiv use_ai={true} /> },
        // Route::CreateOnlineGame => html! { <GameDiv /> },
        // Route::JoinOnlineGame { .. } => html! { <GameDiv /> },
        // Route::NotFound => html! { <GameDiv /> },
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
