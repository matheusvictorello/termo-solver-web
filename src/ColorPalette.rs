use yew::prelude::*;
use web_sys::console;

use crate::termo::*;
use crate::Square::*;

#[derive(Properties, PartialEq)]
pub struct ColorPaletteProps {
    pub status: SquareStatus,
    pub onclick: Callback<SquareStatus>,
}

pub struct ColorPalette {
    pub status:  SquareStatus,
    pub onclick: Callback<SquareStatus>,
}

impl Component for ColorPalette {
	type Message    = ();
    type Properties = ColorPaletteProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        Self {
            status:  props.status,
            onclick: props.onclick.clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();

        self.status  = props.status;
        self.onclick = props.onclick.clone();

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class={"palette"}>
                {
                    for [
                        SquareStatus::Empty,
                        SquareStatus::Right,
                        SquareStatus::Wrong,
                        SquareStatus::Place,
                    ].map(|status| {
                        let onclick = self.onclick.clone();

                        let onclick = Callback::from(move |_| {
                            onclick.emit(status);
                        });

                        let selected = self.status == status;

                        html! {
                            <Square letter={' '} {status} {onclick} {selected} />
                        }
                    })
                }
            </div>
        }
    }
}