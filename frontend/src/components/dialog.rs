use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct DialogProps {
    pub d_ref: NodeRef,
    pub title: String,
    pub contents: String,
    pub onconfirm: Callback<bool>
}

#[function_component]
pub fn Dialog(props: &DialogProps) -> Html {
    let title = props.title.clone();
    let contents = props.contents.clone();
    let d_ref = props.d_ref.clone();
    let cb = props.onconfirm.clone();

    let onconfirm: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent| {
        cb.emit(true);
    });

    html! {
        <dialog ref={d_ref} class="modal">
            <div class="modal-box">
                <h3 class="font-bold text-lg">{title}</h3>
                <p class="py-4">{contents}</p>
                <div class="modal-action">
                <form method="dialog">
                    <button onclick={onconfirm} class="btn btn-warning mr-2">{"Yes"}</button>
                    <button class="btn">{"No"}</button>
                </form>
                </div>
            </div>
        </dialog>
    }
}
