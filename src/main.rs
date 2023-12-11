mod app;
mod components;
mod helpers;
mod contexts;
mod signature;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
