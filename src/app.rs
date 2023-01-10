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
    pub list: Option<Vec<HeadBlog>>,
}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct HeadBlog {
    pub title: AttrValue,
    pub summary: AttrValue,
}

const BLOGS: &'static [&'static [&'static str]] = &[
    &["第一篇博客", "这是第一篇博客的内容"],
    &["第二篇博客", "这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容这是第二篇博客的内容。"],
    &["第三篇博客", "这是第三篇博客的内容"],
    &["第四篇博客", "这是第四篇博客的内容"],
    &["第五篇博客", "这是第五篇博客的内容"],
    &["第六篇博客", "这是第六篇博客的内容"],
    &["第七篇博客", "这是第七篇博客的内容"],
];

#[function_component]
fn Head(props: &HeadProps) -> Html {
    log!(">> 个人联系方式: ycl18@live.com");
    let blogs: Vec<HeadBlog> = BLOGS
        .iter()
        .map(|e| HeadBlog {
            title: AttrValue::from(e[0]),
            summary: AttrValue::from(e[1]),
        })
        .collect();
    let head_state = use_state(move || HeadProps {
        content: props.content.clone(),
        list: Some(blogs),
    });
    html! {
        <ContextProvider<HeadProps> context={head_state.deref().clone()}>
            <BrowserRouter><Switch<Route> render={switch} /></BrowserRouter>
        </ContextProvider<HeadProps>>
    }
}
