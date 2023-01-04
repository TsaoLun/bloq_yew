use yew::prelude::*;
use gloo::console::log;
use stylist::{yew::styled_component, style};

#[styled_component(App)]
pub fn app() -> Html {
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
            <p>{"记一些物事，不一定是对的。"}</p>
        </div>
    }
}