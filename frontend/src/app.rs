use yew::prelude::*;
use super::sidebar::{SideBar};
use super::main_panel::{MainPanel};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="flex">
            <SideBar></SideBar>
            <MainPanel></MainPanel>
        </div>
    }
}
