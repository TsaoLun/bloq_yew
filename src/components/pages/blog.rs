use crate::router::Route;
use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Blog)]
pub fn hello() -> Html {
    let stylesheet = style!(
        r#"
            text-align: center;
            h1 {
                color: orange;
                text-align: center;
                font-size: 60px;
            }
            button {
                color: green;
            }
        "#
    )
    .unwrap();
    let history = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        history.push(&Route::Home);
    });
    html! {
        <div class={stylesheet}>
            <h1>{"Blog"}</h1>
            <button onclick={onclick}>{"Go Home"}</button>
        </div>
    }
}
