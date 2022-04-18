use yew::prelude::*;

use crate::components::generic::center::Center;
use crate::components::generic::clickable::Clickable;
use crate::components::square::Square;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub columns: usize,
    pub onclick: Callback<usize>,
}

#[function_component(ColumnPallete)]
pub fn view(props: &Properties) -> Html {
    let Properties { columns, onclick } = props;

    html! {
        <Center>
            <div class="pallete">
                {
                    for [1 as usize, 2, 4].map(|digit| {
                        let letter = char::from_digit(digit as u32, 10);

                        let selected = *columns == digit;
                        let onclick  = onclick.clone();
                        let onclick  = Callback::from(move |_| onclick.emit(digit));

                        html! {
                            <Clickable {onclick}>
                                <Square {letter} {selected} />
                            </Clickable>
                        }
                    })
                }
            </div>
        </Center>
    }
}