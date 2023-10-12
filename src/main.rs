mod app;
mod components;
mod types;

use app::App;

trait IsNoneOr<T>: Copy {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T: Copy> IsNoneOr<T> for Option<T> {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Some(x) => f(x),
            None => true,
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
