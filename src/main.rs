mod app;
mod components;
mod types;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
