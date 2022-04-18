use yew::prelude::*;
// use web_sys::console;

#[derive(Properties, PartialEq)]
pub struct Properties {
    #[prop_or_default]
    pub class:    Classes,
    #[prop_or_default]
    pub children: Children,
    pub onclick:  Callback<()>,
}

#[function_component(Clickable)]
pub fn clickable(props: &Properties) -> Html {
    let Properties {
        class,
        children,
        onclick,
    } = &props;

    let onclick = onclick.clone();

    html! {
        <div
            class={classes!("clickable", class.clone())}
            onclick={move |_| onclick.emit(())}
        >
            { children.clone() }
        </div>
    }
}