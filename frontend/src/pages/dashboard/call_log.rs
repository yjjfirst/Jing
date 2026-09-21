use yew::prelude::*;
use charming::{component::{Title}, Chart, WasmRenderer};
use charming::{component::Axis, element::AxisType, series::Bar};

#[component]
pub fn CallLogCard() -> Html {
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