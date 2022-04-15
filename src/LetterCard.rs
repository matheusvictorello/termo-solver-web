use yew::prelude::*;
use web_sys::console;

use crate::termo::*;
use crate::TermoGame::*;
use crate::SquareLine::*;
use crate::Square::*;

type LetterCardMsg = (usize, usize);

#[derive(Properties, PartialEq)]
pub struct LetterCardProps {
    pub lines:   usize,
    pub words:         [Option<Word>; MAX_LINES],
    pub status:        [SquarePattern; MAX_LINES],
    pub onsquareclick: Callback<LetterCardMsg>,
}

pub struct LetterCard {
    pub lines:         usize,
    pub words:         [Option<Word>; MAX_LINES],
    pub status:        [SquarePattern; MAX_LINES],
    pub onsquareclick: Callback<LetterCardMsg>,
}

impl Component for LetterCard {
    type Message    = LetterCardMsg;
    type Properties = LetterCardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        Self {
            lines:         props.lines,
            words:         props.words,
            status:        props.status,
            onsquareclick: props.onsquareclick.clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();

        self.lines         = props.lines;
        self.words         = props.words;
        self.status        = props.status;
        self.onsquareclick = props.onsquareclick.clone();

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class={"letter_card"}>
                <div class={"inner"}>
                    {
                        for (0..self.lines).map(|i| {
                            match (self.words.get(i), self.status.get(i)) {
                                (Some(&word), Some(&pattern)) => {
                                    let onsquareclick = link.callback(move |column| (i, column));

                                    html! {
                                        <SquareLine {word} {pattern} {onsquareclick}/>
                                    }
                                }
                                _ => { html! {} }
                            }
                        })
                    }
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.onsquareclick.emit(msg);

        false
    }
}