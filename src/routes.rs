use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::controller::Controller;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    TermoDefault,

    #[at("/:columns")]
    TermoNumbered { columns: usize },
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::TermoDefault => html! {
            <Controller columns={1} />
        },
        Route::TermoNumbered { columns } => html! {
            <Controller columns={*columns} />
        },
    }
}