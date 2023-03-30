use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <TopBanner></TopBanner>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello WASM" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}

#[function_component]
pub fn  TopBanner() -> Html {
    html! {
        <div class="top_banner">
        {"This is top banner"}
        </div>
    }
}
