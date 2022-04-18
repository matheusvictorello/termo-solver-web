use yew::prelude::*;
use yew_router::prelude::*;

use crate::ctx::color_ctx::ColorProvider;
use crate::routes::Route;
use crate::routes::switch;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ColorProvider>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ColorProvider>
    }
}