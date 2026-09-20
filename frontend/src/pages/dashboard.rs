use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::models::Service;
use yewdux::prelude::*;
use crate::store::{Store};

use charming::{component::{Legend, Title}, df, element::{ItemStyle, Label, Tooltip},series::Pie, Chart, WasmRenderer};
use charming::{component::Axis, element::AxisType, series::Bar};
#[function_component]
pub fn MemmoryCard() -> Html {
    let f = yew_hooks::use_async::<_, _, ()>({
        let chart = Chart::new()
        .title(Title::new().text("Memory"))
        .legend(Legend::new().top("bottom"))
        .series(
            Pie::new()                
                .name("Memory Chart")
                .label(Label::new().show(false))
                .radius(vec![30, 80])
                .item_style(ItemStyle::new().border_radius(8))
                .data(df![
                    (80.0, "Used"),
                    (20.0, "Free"),
                ]),
        );

        let renderer = WasmRenderer::new(250, 250);
        
        async move {
            renderer.render("memory_chart", &chart).unwrap();
            Ok(())
        }
    });
    
    use_effect_with((), move |_| {
        f.run();
        || ()
    });

    html! {
        <div id="memory_chart"  class="m-1"></div>
    }
}

#[function_component]
pub fn CpuCard() -> Html {
    let f = yew_hooks::use_async::<_, _, ()>({
        let chart = Chart::new()
        .title(Title::new().text("CPU"))
        .legend(Legend::new().top("bottom"))
        .tooltip(Tooltip::new())
        .series(
            Pie::new()                
                .name("CPU Chart")
                .radius(vec![30, 80])
                .label(Label::new().show(false))
                .item_style(ItemStyle::new().border_radius(8))
                .data(df![
                    (30.0, "Used"),
                    (70.0, "Free"),
                ]),
        );

        let renderer = WasmRenderer::new(250, 250);
        
        async move {
            renderer.render("cpu_chart", &chart).unwrap();
            Ok(())
        }
    });
    
    use_effect_with((), move |_| {
        f.run();
        || ()
    });

    html! {
        <div id="cpu_chart" class="m-1"></div>
    }
}
#[function_component]
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
        .tooltip(Tooltip::new())
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
    
    use_effect_with((), move |_| {
        spawn_local( async move {
            let disk_info: DiskInfo = Service::get("/system-info/disk", store.selected_domain_id)
                .await
                .unwrap();

            used.set(disk_info.used);
            free.set(disk_info.free);
            f.run();
        });
    });

    html! {
        <div id="disk_chart" class="m-1"></div>
    }
}

#[function_component]
pub fn CallStat() -> Html {
        let f = yew_hooks::use_async::<_, _, ()>({
            let chart = Chart::new()
            .title(Title::new().text("Calls"))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Bar::new().data(vec![150, 230, 224, 218, 135, 147, 260]));
    
            let renderer = WasmRenderer::new(900, 400);
            
            async move {
                renderer.render("call_chart", &chart).unwrap();
                Ok(())
            }
        });
        
        use_effect_with((), move |_| {
            f.run();
            || ()
        });
    
        html! {
            <div id="call_chart"></div>
        }
}
#[function_component]
pub fn Dashboard() -> Html {
    html! {
        <div>
            <div class="flex flex-wrap pbx-card justify-center m-2">
                <MemmoryCard />
                <CpuCard />
                <DiskCard />
            </div>        
            <div class="flex flex-wrap pbx-card justify-center m-2">
                <CallStat />
            </div>
        </div>
    }
}
