use yew::prelude::*;

use crate::components::data::block::Block;
use crate::components::square_line::SquareLine;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub block:   Block,
    pub onclick: Callback<(usize, usize)>,
}

#[function_component(BlockCard)]
pub fn view(props: &Properties) -> Html {
    let Properties { block, onclick } = props;
    let Block { lines, entries, .. } = block;

    html! {
        <div class="block_card">
            {
                for (0..*lines).map(|i| {
                    let entry = entries[i];
                    let onclick = onclick.clone();
                    let onclick = Callback::from(move |j| onclick.emit((i, j)));

                    html! {
                        <SquareLine {entry} {onclick} />
                    }
                })
            }
        </div>
    }
}