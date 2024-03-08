mod app;
mod components;
mod constants;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
