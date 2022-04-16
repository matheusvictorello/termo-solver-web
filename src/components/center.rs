use yew::prelude::*;
// use web_sys::console;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub children: Children,
}

#[function_component(Center)]
pub fn center(props: &Properties) -> Html {
    html! {
        <div class="center">
            { for props.children.iter() }
        </div>
    }
}