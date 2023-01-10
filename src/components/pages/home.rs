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
                color: orange;
                display: block;
                font-size: 25px;
            }
            ul {
                jusify-content: center;
                display: flex;
                flex-direction: column;
                padding: 0;
            }
            li {
                color: white;
                margin: 0 auto;
                height: 50px;
            }
            .avatar {
                width: 100px;
                height: 100px;
                background-size: cover;
                border-radius: 9999px;
                margin: auto;
            }
            .title {
                padding-top: 2rem;
            }
            .summary {
                margin: auto;
                width: 900px;
                color: grey;
            }
        "#
    )
    .unwrap();
    let home_context = use_context::<HeadProps>().unwrap_or_default();
    let blog_list: Html = home_context
        .list
        .unwrap_or_default()
        .iter()
        .map(|blog| {
            html! {
                <>
                    <li class="title"><Link<Route> to={Route::Blog}>{blog.title.clone()}</Link<Route>></li>
                    <p class="summary">{blog.summary.clone()}</p>
                </>
            }
        })
        .collect();
    html! {
        <div class={stylesheet}>
            <h1>{"bloq"}</h1>
            <a class="avatar" style="background-image: url(https://deno-avatar.deno.dev/avatar/blog.svg);"></a>
            <p>{home_context.content}</p>
            <ul>
                {blog_list}
            </ul>
        </div>
    }
}
