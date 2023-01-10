use crate::router::{switch, Route};
use gloo::console::log;
use std::ops::Deref;
use stylist::yew::styled_component;
use yew::{prelude::*, virtual_dom::AttrValue};
use yew_router::prelude::*;
#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Head content={"记一些物事，不一定是对的。"}/>
        </>
    }
}
#[derive(Properties, PartialEq, Clone, Default)]
pub struct HeadProps {
    pub content: AttrValue,
    pub list: Option<Vec<AttrValue>>,
}

#[function_component]
fn Head(props: &HeadProps) -> Html {
    log!(">> 个人联系方式: ycl18@live.com");
    let head_state = use_state(|| HeadProps {
        content: props.content.clone(),
        list: Some(vec![AttrValue::from("001"), AttrValue::from("002")]),
    });
    html! {
        <ContextProvider<HeadProps> context={head_state.deref().clone()}>
            <BrowserRouter><Switch<Route> render={switch} /></BrowserRouter>
        </ContextProvider<HeadProps>>
    }
}
