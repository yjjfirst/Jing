use yew::prelude::*;
use yew_hooks::use_interval;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::models::Service;
use yewdux::prelude::*;
use crate::store::{Store};

use charming::{component::{Title}, df, element::{ItemStyle, Label, Tooltip},series::Pie, Chart, WasmRenderer};

#[component]
pub fn LoadingCard() -> Html {
    #[derive(Deserialize, Serialize)]
    pub struct Loading {
        pub one: f64,
    }

    let loading = use_state(||0.0);
    let (store, _) = use_store::<Store>();
    let f = yew_hooks::use_async::<_, _, ()>({
        let chart = Chart::new()
        .title(Title::new().text("Loading"))
        .tooltip(Tooltip::new())
        .series(
            Pie::new()
                .name("Loading")
                .radius(vec![30, 80])
                .label(Label::new().show(false))
                .item_style(ItemStyle::new().border_radius(8))
                .data(df![
                    (*loading, "Used"),
                    (((1.0 - *loading) * 100f64).floor() / 100.0, "Free"),
                ]),
        );

        let renderer = WasmRenderer::new(250, 250);

        async move {
            renderer.render("loading_chart", &chart).unwrap();
            Ok(())
        }
    });

    {
        let loading = loading.clone();
        let f = f.clone();
        let store = store.clone();
        use_effect_with((), move |_| {
            let loading = loading.clone();
            let f = f.clone();
            let store = store.clone();
            spawn_local( async move {
                let loading_fetched: Loading = Service::get("/system-info/loading", store.selected_domain_id)
                    .await
                    .unwrap();
                loading.set(loading_fetched.one);
                f.run();
            });
        });
    }

    {
        let loading = loading.clone();
        let f = f.clone();
        let store = store.clone();

        use_interval(move || {
            let loading = loading.clone();
            let f = f.clone();
            let store = store.clone();
            spawn_local( async move {
                let loading_fetched: Loading = Service::get("/system-info/loading", store.selected_domain_id)
                    .await
                    .unwrap();
                loading.set(loading_fetched.one);
                f.run();
            });
        }, 10 * 1000);
    }
    html! {
        <div id="loading_chart" class="m-1"></div>
    }
}