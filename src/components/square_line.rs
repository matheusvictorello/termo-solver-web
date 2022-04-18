use yew::prelude::*;

use crate::components::data::entry::Entry;
use crate::components::generic::clickable::Clickable;
use crate::components::square::Square;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub entry: Entry,
    pub onclick: Callback<usize>,
}

#[function_component(SquareLine)]
pub fn view(props: &Properties) -> Html {
    let Properties { entry, onclick } = props;

    match entry {
        Entry::Unset => {
            html! {
                <div class="square_line">
                    {
                        for (0..5).map(|_| {
                            html! {
                                <Square />
                            }
                        })
                    }
                </div>
            }
        }
        Entry::Editable(word, pattern) => {
            html! {
                <div class="square_line">
                    {
                        for (0..5).map(|i| {
                            let letter = word.0[i];
                            let status = pattern[i];
                            let onclick = onclick.clone();
                            let onclick = Callback::from(move |_| onclick.emit(i));

                            html! {
                                <Clickable {onclick}>
                                    <Square {status} {letter} />
                                </Clickable>
                            }
                        })
                    }
                </div>
            }
        }
        Entry::Fixed((word, pattern)) => {
            html! {
                <div class="square_line">
                    {
                        for (0..5).map(|i| {
                            let letter = word.0[i];
                            let status = pattern.0[i];
                            
                            html! {
                                <Square {status} {letter} />
                            }
                        })
                    }
                </div>
            }
        }
    }
}