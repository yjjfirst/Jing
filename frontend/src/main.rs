mod app;
mod sidebar;
mod main_panel;
mod ringing_group;
mod cards;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
