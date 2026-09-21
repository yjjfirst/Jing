use yew::prelude::*;
use yew_hooks::use_interval;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::models::Service;
use yewdux::prelude::*;
use crate::store::{Store};

use charming::{component::{Title}, df, element::{ItemStyle, Label, Tooltip},series::Pie, Chart, WasmRenderer};

#[component]
pub fn DiskCard() -> Html {
    #[derive(Deserialize, Serialize)]
    pub struct DiskInfo {
        pub used: i32,
        pub free: i32,
    }

    let used = use_state(||0);
    let free = use_state(||0);
    let (store, _) = use_store::<Store>();
    let f = yew_hooks::use_async::<_, _, ()>({
        let used = used.clone();
        let free = free.clone();
        let chart = Chart::new()
        .title(Title::new().text("Disk"))
        .tooltip(Tooltip::new()
            .formatter("{a}<br/> {b}: {c}G")
        )
        .series(
            Pie::new()
                .name("Disk")
                .radius(vec![30, 80])
                .label(Label::new().show(false))
                .item_style(ItemStyle::new().border_radius(8))
                .data(df![
                    (*used, "Used"),
                    (*free, "Free"),
                ]),
        );

        let renderer = WasmRenderer::new(250, 250);

        async move {
            renderer.render("disk_chart", &chart).unwrap();
            Ok(())
        }
    });
    {
        let used = used.clone();
        let free = free.clone();
        let f = f.clone();
        let store = store.clone();
        use_effect_with((), move |_| {
            let used = used.clone();
            let free = free.clone();
            let f = f.clone();
            let store = store.clone();
            spawn_local( async move {
                let disk_info: DiskInfo = Service::get("/system-info/disk", store.selected_domain_id)
                    .await
                    .unwrap();

                used.set(disk_info.used);
                free.set(disk_info.free);
                f.run();
            });
        });
    }

    {
        let used = used.clone();
        let free = free.clone();
        let f = f.clone();
        let store = store.clone();
        use_interval(move || {
            let used = used.clone();
            let free = free.clone();
            let f = f.clone();
            let store = store.clone();
            spawn_local( async move {
                let disk_info: DiskInfo = Service::get("/system-info/disk", store.selected_domain_id)
                    .await
                    .unwrap();

                used.set(disk_info.used);
                free.set(disk_info.free);
                f.run();
            });
        }, 120*1000);
    }
    html! {
        <div id="disk_chart" class="m-1"></div>
    }
}
