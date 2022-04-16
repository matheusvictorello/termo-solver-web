use termo_solver::Status as TSStatus;
use yew::prelude::*;
// use web_sys::console;

use crate::components::center::Center;
use crate::components::clickable::Clickable;
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
            <div class="pallete">
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
            </div>
        </Center>
    }
}