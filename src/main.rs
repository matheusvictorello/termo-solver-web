use yew::prelude::*;
use yew_router::prelude::*;
// use web_sys::console;

mod components;

use components::controller::Controller;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    #[not_found]
    TermoDefault,

    #[at("/:columns")]
    Termo { columns: usize },
}

fn switch(route: &Route) -> Html {
    match route {
        Route::TermoDefault => html! {
            <Controller columns={1} />
        },
        Route::Termo { columns } => html! {
            <Controller columns={columns.clone()} />
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}