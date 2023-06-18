use yew::prelude::*;
use yew_router::prelude::*;
use super::sidebar::{SideBar};
use super::main_panel::{MainPanel};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="flex">
                <SideBar></SideBar>
                <MainPanel></MainPanel>
            </div>
        </BrowserRouter>
    }
}
