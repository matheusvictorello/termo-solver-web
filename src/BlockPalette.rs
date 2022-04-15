use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::console;

use crate::Route;
use crate::termo::*;
use crate::Square::*;

#[derive(Properties, PartialEq)]
pub struct BlockPaletteProps {
    pub columns: usize,
    pub onclick: Callback<usize>,
}

pub struct BlockPalette {
    pub columns: usize,
    pub onclick: Callback<usize>,
}

impl Component for BlockPalette {
	type Message    = ();
    type Properties = BlockPaletteProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        Self {
            columns: props.columns,
            onclick: props.onclick.clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();

        self.columns = props.columns;
        self.onclick = props.onclick.clone();

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class={"palette"}>
                {
                    for [
                        1, 2, 4
                    ].map(|columns| {
                        let letter = char::from_digit(columns as u32, 10).unwrap();
                        
                        let onclick = self.onclick.clone();

                        let onclick = Callback::from(move |_| {
                            onclick.emit(columns);
                        });

                        let selected = self.columns == columns;

                        html! {
                            <Link<Route> classes={classes!("no_underline")} to={Route::Termo { columns }}>
                                <Square {letter} {onclick} {selected} />
                            </Link<Route>>
                        }
                    })
                }
            </div>
        }
    }
}