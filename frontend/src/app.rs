use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="flex">
            <div>
            <img/>
            </div>
            <div>
                <h1>{"Welcome to NorthBy"}</h1>
                <h2>{"A premium in sight and sound"}</h2>
                <button>{"Learn More"}</button>
            </div>
        </div>
    }
}

#[function_component]
pub fn  TopBanner() -> Html {
    let banner = "This is top banner";
    html! {
        <div class="top_banner">
        {banner}
        </div>
    }
}
