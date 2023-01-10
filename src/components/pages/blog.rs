use crate::router::Route;
use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Blog)]
pub fn hello() -> Html {
    let stylesheet = style!(
        r#"
            h1 {
                color: white;
                text-align: center;
                font-size: 40px;
            }
            button {
                color: white;
                width: 4rem;
                height: 4rem;
                margin-top: 1rem;
                margin-left: 1rem;
                text-align: left;
                background-color: darkorange;
                border-width: 0px;
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
            <button onclick={onclick}>{"Go Home"}</button>
            <h1>{"Blog"}</h1>
        </div>
    }
}
