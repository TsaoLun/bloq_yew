use crate::{app::HeadProps, router::Route};
use stylist::{style, yew::styled_component};
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
        "#
    )
    .unwrap();
    let home_context = use_context::<HeadProps>();
    html! {
        <div class={stylesheet}>
            <h1>{"Home"}</h1>
            <h1>{"bloq"}</h1>
            <p>{home_context.unwrap_or_default().content}</p>
            <Link<Route> to={Route::Blog}>{"To Blog"}</Link<Route>>
        </div>
    }
}
