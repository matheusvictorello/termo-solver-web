use yew::prelude::*;
use web_sys::console;

use crate::termo::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SquareStatus {
    Right,
    Wrong,
    Place,
    Empty,
}

#[derive(Properties, PartialEq)]
pub struct SquareProps {
    pub letter:   Option<char>,
    pub status:   Option<SquareStatus>,
    pub onclick:  Option<Callback<()>>,
    pub selected: Option<bool>,
}

pub struct Square {
    pub letter:   Option<char>,
    pub status:   Option<SquareStatus>,
    pub onclick:  Option<Callback<()>>,
    pub selected: Option<bool>,
}

impl Component for Square {
	type Message    = ();
    type Properties = SquareProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        Self {
            letter:   props.letter,
            status:   props.status,
            onclick:  props.onclick.clone(),
            selected: props.selected,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();

        self.letter   = props.letter;
        self.status   = props.status;
        self.onclick  = props.onclick.clone();
        self.selected = props.selected;


        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let letter = match self.letter {
            Some(l) => l.to_uppercase().collect::<String>(),
            None    => String::from(" "),
        };

        let blank = match self.letter {
            Some(l) => None,
            None    => Some("blank"),
        };
        
        let status = match self.status {
            Some(SquareStatus::Right) => Some("right"),
            Some(SquareStatus::Wrong) => Some("wrong"),
            Some(SquareStatus::Place) => Some("place"),
            Some(SquareStatus::Empty) => None,
            None => None,
        };

        let selected = match self.selected {
            Some(true)  => Some("selected"),
            Some(false) => None,
            None        => None,
        };

        let onclick = match (self.onclick.clone(), self.letter) {
            (Some(onclick), Some(_)) => Callback::from(move |_| onclick.emit(())),
            _                        => Callback::from(move |_| {},),
        };

        let class = classes!("square", status, blank, selected);

        html! {
            <div {class} {onclick}>{letter}</div>
        }
    }
}