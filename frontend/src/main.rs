mod app;
mod models;
mod components;
mod pages;
mod store;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
