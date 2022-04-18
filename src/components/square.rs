use termo_solver::Status as TSStatus;

use yew::prelude::*;

use crate::ctx::color_ctx::Color;
use crate::ctx::color_ctx::ColorContext;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub status:   Option<TSStatus>,
    pub letter:   Option<char>,
    pub selected: Option<bool>,
}

#[function_component(Square)]
pub fn view(props: &Properties) -> Html {
    let Properties { status, letter, selected } = props;

    let color_ctx = use_context::<ColorContext>().unwrap();

    let status = match status {
        Some(TSStatus::Right) => match *color_ctx {
            Color::Default    => Some("right"),
            Color::Colorblind => Some("right_colorblind"),
        },
        Some(TSStatus::Wrong) => Some("wrong"),
        Some(TSStatus::Place) => match *color_ctx {
            Color::Default    => Some("place"),
            Color::Colorblind => Some("place_colorblind"),
        },
        None                  => match letter {
            Some(_) => None,
            None    => Some("blank"),
        },
    };

    let letter = match letter {
        Some(letter) => *letter,
        None         => ' ',
    };

    let selected = match selected {
        Some(true)  => Some("selected"),
        Some(false) => None,
        None        => None,
    };

    html! {
        <div class={classes!("square", status, selected)}>
            {letter}
        </div>
    }
}