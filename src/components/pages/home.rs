use crate::{router::Route, app::HeadProps};
use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{style, yew::styled_component};

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
        "#
    )
    .unwrap();
    let home_context = use_context::<HeadProps>();
    html! {
        <div>
            <h1>{"Home"}</h1>
            <div class={stylesheet}>
            <h1>{"bloq"}</h1>
            <p>{home_context.unwrap_or_default().content}</p>
                <Link<Route> to={Route::Blog}>{"To Blog"}</Link<Route>>
            </div>
        </div>
    }
}