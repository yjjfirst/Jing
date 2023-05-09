mod app;
mod sidebar;
mod main_panel;
mod ringing_group;
mod cards;
mod header;
mod button;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
