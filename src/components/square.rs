use termo_solver::Status as TSStatus;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub status:   Option<TSStatus>,
    pub letter:   Option<char>,
    pub selected: Option<bool>,
}

#[function_component(Square)]
pub fn view(props: &Properties) -> Html {
    let Properties { status, letter, selected } = props;

    let status = match status {
        Some(TSStatus::Right) => Some("right"),
        Some(TSStatus::Wrong) => Some("wrong"),
        Some(TSStatus::Place) => Some("place"),
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