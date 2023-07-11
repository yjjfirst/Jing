use yew::prelude::*;
use yew_router::prelude::*;
use super::components::sidebar::{SideBar};
use super::main_panel::{MainPanel};

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    pub base_url: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_state (|| Env {
        base_url: "http://teleman.me:9090/api".to_owned(),
    });

    html! {
        <ContextProvider<Env> context={(*ctx).clone()}>
            <BrowserRouter>
                <div class="flex">
                    <SideBar></SideBar>
                    <MainPanel></MainPanel>
                </div>
            </BrowserRouter>
        </ContextProvider<Env>>
    }
}
