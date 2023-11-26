mod app;
mod components;
mod helpers;
mod contexts;
mod eip1193;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
