use yew::prelude::*;

use crate::components::generic::clickable::Clickable;

#[derive(Properties, PartialEq)]
pub struct Properties {
    #[prop_or_default]
    pub class:   Classes,
    pub set:     bool,
    pub onclick: Callback<bool>,
}

#[function_component(Switch)]
pub fn view(props: &Properties) -> Html {
    let Properties { class, set, onclick, .. } = props;

    let set = *set;

    let switch_set_class = if set {
        Some("switch_set")
    } else {
        None
    };

    let inner_switch_set_class = if set {
        Some("inner_switch_set")
    } else {
        None
    };

    let onclick = onclick.clone();
    let onclick = Callback::from(move |_| onclick.emit(!set));

    html! {
        <Clickable {onclick} class={class.clone()}>
            <div class={classes!("switch", switch_set_class)}>
                <div class={classes!("inner_switch", inner_switch_set_class)} />
            </div>
        </Clickable>
    }
}