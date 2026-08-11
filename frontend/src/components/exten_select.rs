use yew::prelude::*;
use crate::store::Store;
use yewdux::prelude::*;
use crate::models::extension::Extension;
use crate::models::Service;
use std::collections::BTreeMap;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub value: String,
    #[prop_or(classes!("col-span-2"))]
    pub classes: Classes
}

#[function_component]
pub fn ExtenionSelect(props: &Props) -> Html {
    let id = props.id.clone();
    let name= id.clone();
    let value = props.value.clone();

    let(store,_) = use_store::<Store>();
    let ext_map: UseStateHandle<BTreeMap<String, Vec<String>>> = use_state(||BTreeMap::new());
    {
        let ext_map = ext_map.clone();
        use_effect_with((), move |_|{
            let ext_map = ext_map.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let ext_map = ext_map.clone();
                let mut fetched_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
                let url = format!("/extension");
                let extensions: Vec<Extension> = Service::index(&url, store.selected_domain_id).await.unwrap();
                for e in extensions {
                    if !fetched_map.contains_key(&e.exten_type) {
                        fetched_map.insert(e.exten_type.clone(), vec![e.exten.clone()]);
                    } else {
                        let exist_exten = fetched_map.get_mut(&e.exten_type).unwrap();
                        exist_exten.push(e.exten.clone());
                    }
                }
                ext_map.set(fetched_map);
            })
        });
    }

    let options_list: Vec<Html> = ext_map.iter().map(|(k,v)|{
        let e_list: Vec<Html> =
            v.into_iter().map(|e|{
                html! {
                    if e.eq(&props.value) {
                        <option value={e.clone()} selected=true>{e.clone()}</option>
                    } else {
                        <option value={e.clone()}>{e.clone()}</option>
                    }
                }
            }).collect();

        html! {
            <optgroup label={k.clone()}>
                {e_list}
            </optgroup>
        }
    }).collect();

    let classes = classes!("select", "select-bordered", "w-full", props.classes.clone());
    html! {
        <select class={classes} name={name} value={value.clone()} id={id}>
            if value == "" {
                <option value="" disabled={true} selected={true} hidden={true}>{"Select a extension"}</option>
            }
            {options_list}
        </select>
    }
}
