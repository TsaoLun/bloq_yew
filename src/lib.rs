use yew::{prelude::*, virtual_dom};
use gloo::console::log;
use stylist::{yew::styled_component, style};

#[styled_component(App)]
pub fn app() -> Html {
    html!{
        <>
            <Head content={"记一些物事，不一定是对的。"}/>
        </>
    }
}
#[derive(Properties, PartialEq)]
pub struct HeadProps {
    pub content: virtual_dom::AttrValue,
}

#[function_component]
fn Head(props: &HeadProps) -> Html {
    log!(">> 个人联系方式: ycl18@live.com");
    let stylesheet = style!(
        r#"
            h1 {
                color: orange;
                text-align: center;
                font-size: 60px;
            }
            p {
                color: white;
                text-align: center;
            }
        "#
    ).unwrap();
    html!{
        <div class={stylesheet}>
            <h1>{"bloq"}</h1>
            <p>{props.content.clone()}</p>
        </div>
    }
}