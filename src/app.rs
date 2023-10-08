use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="mt-12 text-[#fff6d5] font-serif text-2xl text-center">
            <img class="h-[20em]" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="text-slate-900">{ "from Yew with Tailwind hooked up" }</span>
        </main>
    }
}
