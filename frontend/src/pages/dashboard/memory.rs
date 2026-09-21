use yew::prelude::*;
use yew_hooks::use_interval;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::models::Service;
use yewdux::prelude::*;
use crate::store::{Store};

use charming::{component::{Title}, df, element::{ItemStyle, Label, Tooltip},series::Pie, Chart, WasmRenderer};
#[component]
pub fn MemoryCard() -> Html {
    #[derive(Serialize, Deserialize, Clone)]
    pub struct MemInfo {
        pub free: f64,
        pub used: f64,
        pub avail: f64,
    }
    let mem_info = use_state(||MemInfo{
        free: 0.0, used: 0.0, avail: 0.0
    });
    let (store,_) = use_store::<Store>();
    let f = yew_hooks::use_async::<_, _, ()>({
        let mem_info = mem_info.clone();
        let chart = Chart::new()
        .title(Title::new().text("Memory"))
        .tooltip(Tooltip::new()
            .formatter("{a}<br/> {b}: {c}M")
        )
        .series(
            Pie::new()
                .name("Memory")
                .label(Label::new().show(false))
                .radius(vec![30, 80])
                .item_style(ItemStyle::new().border_radius(8))
                .data(df![
                    (((*mem_info).used/1000.0).trunc(), "Used"),
                    (((*mem_info).free/1000.0).trunc(), "Free"),
                    (((*mem_info).avail/1000.0).trunc(), "Avail"),
                ]),
        );

        let renderer = WasmRenderer::new(250, 250);

        async move {
            renderer.render("memory_chart", &chart).unwrap();
            Ok(())
        }
    });
    {
        let mem_info = mem_info.clone();
        let store = store.clone();
        let f = f.clone();
        use_effect_with((), move |_| {
            let mem_info = mem_info.clone();
            let store = store.clone();
            let f = f.clone();
            spawn_local(async move{
                let mem_fetched: MemInfo = Service::get("/system-info/memory", store.selected_domain_id)
                    .await
                    .unwrap();
                mem_info.set(mem_fetched);
                f.run()
            });
        });
    }
    {
        let mem_info = mem_info.clone();
        let store = store.clone();
        let f = f.clone();
        use_interval(move || {
            let mem_info = mem_info.clone();
            let store = store.clone();
            let f = f.clone();
            spawn_local(async move{
                let mem_fetched: MemInfo = Service::get("/system-info/memory", store.selected_domain_id)
                    .await
                    .unwrap();
                mem_info.set(mem_fetched);
                f.run()
            });
        }, 10 * 1000)
    }
    html! {
        <div id="memory_chart"  class="m-1"></div>
    }
}
