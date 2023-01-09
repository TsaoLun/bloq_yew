use crate::router::{switch, Route};
use gloo::console::log;
use stylist::{style, yew::styled_component};
use yew::{prelude::*, virtual_dom};
use yew_router::prelude::*;
use crate::components::pages::home::Home;
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
    pub content: virtual_dom::AttrValue,
}

#[function_component]
fn Head(props: &HeadProps) -> Html {
    log!(">> 个人联系方式: ycl18@live.com");
    let head_state = use_state(||HeadProps{content: props.content});
    html! {
        <ContextProvider<HeadProps> context={head_state.deref().clone()}>
            <Home>
            <BrowserRouter><Switch<Route> render={switch} /></BrowserRouter>
        </ContextProvider>
    }
}
