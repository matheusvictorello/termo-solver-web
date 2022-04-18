use yew::prelude::*;
// use web_sys::console;

#[derive(Properties, PartialEq)]
pub struct Properties {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Center)]
pub fn center(props: &Properties) -> Html {
    let Properties {
        class,
        children,
    } = &props;

    html! {
        <div class={classes!("center", class.clone())}>
            { children.clone() }
        </div>
    }
}