use self::{home::Home, how_to_play::HowToPlay};
use crate::components::{game_div::AITask, AIGameDiv, LMGameDiv};
use yew::prelude::*;
use yew_agent::oneshot::OneshotProvider;
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
        Route::LocalGame => html! { <LMGameDiv /> },
        Route::AiGame => html! {
            <OneshotProvider<AITask> path="./worker.js">
                <AIGameDiv />
            </OneshotProvider<AITask>>
        },
        // Route::CreateOnlineGame => html! { <GameDiv /> },
        // Route::JoinOnlineGame { .. } => html! { <GameDiv /> },
        // Route::NotFound => html! { <GameDiv /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}
