mod app;
mod main_panel;
mod cards;
mod services;
mod components;
mod pages;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
