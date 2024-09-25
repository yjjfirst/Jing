
use yew::prelude::*;
use charming::{component::{Legend, Title}, df, element::{ItemStyle, Label},series::Pie, Chart, WasmRenderer};
use charming::{component::Axis, element::AxisType, series::Bar};
#[function_component]
pub fn MemmoryCard() -> Html {
    let f = yew_hooks::use_async::<_, _, ()>({
        let chart = Chart::new()
        .title(Title::new().text("Memory Usage"))
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
        <div id="memory_chart"></div>
    }
}

#[function_component]
pub fn CpuCard() -> Html {
    let f = yew_hooks::use_async::<_, _, ()>({
        let chart = Chart::new()
        .title(Title::new().text("CPU Usage"))
        .legend(Legend::new().top("bottom"))
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
        <div id="cpu_chart"></div>
    }
}
#[function_component]
pub fn DiskCard() -> Html {
    let f = yew_hooks::use_async::<_, _, ()>({
        let chart = Chart::new()
        .title(Title::new().text("Disk Usage"))
        .legend(Legend::new().top("bottom"))
        .series(
            Pie::new()                
                .name("Disk Chart")
                .radius(vec![30, 80])
                .label(Label::new().show(false))
                .item_style(ItemStyle::new().border_radius(8))
                .data(df![
                    (40.0, "Used"),
                    (60.0, "Free"),
                ]),
        );

        let renderer = WasmRenderer::new(250, 250);
        
        async move {
            renderer.render("disk_chart", &chart).unwrap();
            Ok(())
        }
    });
    
    use_effect_with((), move |_| {
        f.run();
        || ()
    });

    html! {
        <div id="disk_chart"></div>
    }
}

#[function_component]
pub fn CallStat() -> Html {
        let f = yew_hooks::use_async::<_, _, ()>({
            let chart = Chart::new()
            .title(Title::new().text("Call Statistics"))
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
        <div class="flex flex-wrap">
            <div class="w-1/3 bg-base-100 shadow-xl">
                <div class="card-body">
                    <MemmoryCard />
                </div>
            </div>
            <div class="w-1/3 bg-base-100 shadow-xl">
                <div class="card-body">
                    <CpuCard />
                </div>
            </div>
            <div class="w-1/3 bg-base-100 shadow-xl">
                <div class="card-body">
                    <DiskCard />
                </div>
            </div>
        </div>
        <div class="w-3/3 bg-base-100 shadow-xl">
            <div class="card-body">
                <CallStat />
            </div>
        </div>
        </div>
    }
}
