use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn start() -> Html {
    let navigator = use_navigator().unwrap();
    let how_to_play_onclick = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::HowToPlay))
    };
    let local_game_onclick = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::LocalGame))
    };
    let ai_game_onclick = Callback::from(move |_| navigator.push(&Route::AiGame));

    html! {
        <div class="flex flex-col mx-auto mt-12 max-w-md text-center gap-3 items-center">
            <h1 class="text-7xl font-bold">{"Super"}</h1>
            <h1 class="text-4xl font-bold mb-5">{"Tic-Tac-Toe"}</h1>
            <button class="font-semibold text-sm bg-primary rounded-full shadow-sm px-4 py-2 max-w-fit" onclick={how_to_play_onclick}>{"How to Play"}</button>
            <button class="font-semibold text-sm bg-primary rounded-full shadow-sm px-4 py-2 max-w-fit" onclick={local_game_onclick}>{"Play Local Multiplayer"}</button>
            <button class="font-semibold text-sm bg-primary rounded-full shadow-sm px-4 py-2 max-w-fit" onclick={ai_game_onclick}>{"Play Against AI"}</button>
        </div>
    }
}
