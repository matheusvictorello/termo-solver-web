use yew::prelude::*;
// use web_sys::console;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub children: Children,
    pub onclick: Callback<()>,
}

#[function_component(Clickable)]
pub fn clickable(props: &Properties) -> Html {
    let onclick = props.onclick.clone();

    html! {
        <div class="clickable" onclick={move |_| onclick.emit(())} >
            { for props.children.iter() }
        </div>
    }
}