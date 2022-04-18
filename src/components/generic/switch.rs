use yew::prelude::*;

use crate::components::generic::center::Center;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub set: bool,
}

#[function_component(Switch)]
pub fn view(props: &Properties) -> Html {
    let Properties { set } = props;

    let set = if *set {
        Some("inner-switch")
    } else {
        Some("inner-switch inner-switch-set")
    };

    html! {
        <Center>
            <div class="pallete">
                <div class="switch">
                    <div class={classes!(set)}>
                    </div>
                </div>
            </div>
        </Center>
    }
}