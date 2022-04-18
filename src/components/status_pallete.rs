use termo_solver::Status as TSStatus;

use yew::prelude::*;

use crate::components::generic::center::Center;
use crate::components::generic::clickable::Clickable;
use crate::components::square::Square;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub status: Option<TSStatus>,
    pub onclick: Callback<Option<TSStatus>>,
}

#[function_component(StatusPallete)]
pub fn view(props: &Properties) -> Html {
    let Properties { status: sel_status, onclick } = props;

    html! {
        <Center>
            {
                for [
                    None,
                    Some(TSStatus::Right),
                    Some(TSStatus::Wrong),
                    Some(TSStatus::Place),
                ].map(|status| {
                    let selected = *sel_status == status;
                    let onclick = onclick.clone();
                    let onclick = Callback::from(move |_| onclick.emit(status));

                    html! {
                        <Clickable {onclick}>
                            <Square letter={' '} {status} {selected} />
                        </Clickable>
                    }
                })
            }
        </Center>
    }
}