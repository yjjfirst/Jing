use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{MouseEvent};
use crate::main_panel::{Route};
use crate::header::{Header};

#[function_component] 
pub fn Cards() -> Html {
    html! {
        <div class="flex flex-wrap">
            <Card></Card>
        </div>
    }
}

#[function_component]
pub fn Card() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_: MouseEvent| navigator.push(&Route::RingingGroups));
    html! {
        <div class="grow">
            <Header title="Application"></Header>
            <div {onclick} class="flex flex-col justify-center bg-white w-56 h-48 hover:bg-zinc-200 duration-500 transition-colors shadow-md">
                <div class="flex justify-center">
                    {"Ring Groups"}
                </div>
            </div>
        </div>
    }    
}
