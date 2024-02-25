use std::ops::Deref;

use yew::prelude::*;
use yew_icons::{Icon, IconId};
use super::label::Label;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label_width_class: Classes,
    pub exists: Vec<String>,
    pub all: Vec<String>
}

#[function_component]
pub fn Mselect(props: &Props) -> Html {
    let class: Classes = props.label_width_class.clone();
    let exists = props.exists.clone();
    let all = props.all.clone();
    let news: UseStateHandle<Vec<String>> = use_state(||vec![]);

    let onclick = {
        let news = news.clone();
        Callback::from(move |_e: MouseEvent|{
            let mut items: Vec<String> = vec![];
            for n in news.deref() {
                items.push(n.to_string());
            }
            items.push("".to_string());
            news.set(items);
        })
    };

    let options = | sel: String| -> Html 
    {
        html!{
            all.iter().map(|i|{
                html!{
                    if sel.eq(i) {
                        <option selected=true>{i}</option>
                    } else {
                        <option>{i}</option>
                    }
                }
            }).collect::<Html>()}
    };

    html! {
        <div class="flex w-full px-3">
            <Label class={class}>
            <span class="label-text">
                {"Extensions"}
            </span>
            </Label>
            <div class="flex flex-col">
            {exists.iter().map(|o|{
                html!{
                    <select class="select select-bordered w-full" name="members">
                    <option></option>
                    {options(o.to_string())}
                    </select>
                }
            }).collect::<Html>()}
            {news.iter().map(|o|{
                html!{
                    <select class="select select-bordered w-full" name="members">
                    <option></option>
                    {options(o.to_string())}
                    </select>
                }
            }).collect::<Html>()}

            <div>
                <div class="btn btn-link btn-sm mr-4" onclick={onclick}>
                    <Icon icon_id={IconId::LucidePlus}/>
                    {"Add"}
                </div>
            </div>
            </div>
        </div>
    }
}