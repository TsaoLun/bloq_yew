use crate::{app::HeadProps, router::Route};
use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
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
            a {
                color: green;
                text-align: center;
                display: block;
            }
            ul {
                jusify-content: center;
                display: flex;
                flex-direction: column;
                padding: 0;
            }
            li {
                color: green;
                margin: 0 auto;
            }
        "#
    )
    .unwrap();
    let home_context = use_context::<HeadProps>().unwrap_or_default();
    html! {
        <div class={stylesheet}>
            <h1>{"bloq"}</h1>
            <p>{home_context.content}</p>
            <ul>
                {home_context.list.unwrap_or_default().iter().map(|id| html!{<li><Link<Route> to={Route::Blog}>{id}</Link<Route>></li>}).collect::<Html>()}
            </ul>
        </div>
    }
}
