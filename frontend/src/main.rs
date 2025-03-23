mod app;
mod models;
mod components;
mod pages;
mod store;
mod utils;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
