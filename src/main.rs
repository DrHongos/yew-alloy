mod app;
mod components;
mod helpers;
mod contexts;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
