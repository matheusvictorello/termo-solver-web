use std::rc::Rc;

use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Default,
    Colorblind,
}

impl Reducible for Color {
    type Action = Color;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.into()
    }
}

pub type ColorContext = UseReducerHandle<Color>;

#[derive(Debug, Properties, PartialEq)]
pub struct Properties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ColorProvider)]
pub fn view(props: &Properties) -> Html {
    let color = use_reducer(|| Color::Default);

    html! {
        <ContextProvider<ColorContext> context={color}>
            { props.children.clone() }
        </ContextProvider<ColorContext>>
    }
}