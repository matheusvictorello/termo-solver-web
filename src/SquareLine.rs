use yew::prelude::*;
use web_sys::console;

use crate::termo::*;
use crate::TermoGame::*;
use crate::Square::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SquarePattern(pub [SquareStatus; 5]);

impl<'a> Into<Option<Pattern>> for &'a SquarePattern {
    fn into(self: &'a SquarePattern) -> Option<Pattern> {
        let mut inner = [Status::Right; 5];

        for i in 0..5 {
            inner[i] = match self.0[i] {
                SquareStatus::Right => Status::Right,
                SquareStatus::Wrong => Status::Wrong,
                SquareStatus::Place => Status::Place,
                SquareStatus::Empty => return None,
            }
        }

        Some(Pattern(inner))
    }
}

type SquareLineMsg = usize;

#[derive(Properties, PartialEq)]
pub struct SquareLineProps {
    pub word:          Option<Word>,
    pub pattern:       Option<SquarePattern>,
    pub onsquareclick: Callback<SquareLineMsg>,
}

pub struct SquareLine {
    pub word:          Option<Word>,
    pub pattern:       Option<SquarePattern>,
    pub onsquareclick: Callback<SquareLineMsg>,
}

impl Component for SquareLine {
    type Message    = SquareLineMsg;
    type Properties = SquareLineProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        
        Self {
            word:          props.word,
            pattern:       props.pattern,
            onsquareclick: props.onsquareclick.clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();

        self.word          = props.word;
        self.pattern       = props.pattern;
        self.onsquareclick = props.onsquareclick.clone();

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let word = match self.word {
            Some(word) => [
                Some(word.0[0]),
                Some(word.0[1]),
                Some(word.0[2]),
                Some(word.0[3]),
                Some(word.0[4]),
            ],
            None => [None; 5],
        };

        let pattern = self.pattern.or_else(
            || Some(SquarePattern([
                SquareStatus::Empty,
                SquareStatus::Empty,
                SquareStatus::Empty,
                SquareStatus::Empty,
                SquareStatus::Empty,
            ]))
        ).unwrap();

        let pairs = word.iter().zip(pattern.0.iter());

        html! {
            <div class="square_line">
                {
                    for pairs.enumerate().map(|(i, (&letter, &status))| {
                        let onclick = link.callback(move |_| i);

                        html! {
                            <Square
                                {letter}
                                {status}
                                {onclick}
                            />
                        }
                    })
                }
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.onsquareclick.emit(msg);

        false
    }
}