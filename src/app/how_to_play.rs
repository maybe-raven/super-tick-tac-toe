use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(HowToPlay)]
pub fn how_to_play() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.back());

    html! {
        <div class="flex flex-col mx-auto mt-12 max-w-md text-center gap-3 items-center">
            <h1>{ "Fucking Google it you dumbass" }</h1>
            <button class="font-semibold text-sm bg-cyan-500 rounded-full shadow-sm px-4 py-2 max-w-fit" onclick={onclick}>{"Back"}</button>
        </div>
    }
}