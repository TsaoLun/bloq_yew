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
                color: darkorange;
                display: block;
                font-size: 25px;
                text-decoration: none;
            }
            ul {
                padding: 0;
                border-top-width: 1px;
                border-color: rgba(209,213,219,0.8);
                border-top-style: solid;
                width: 650px;
                margin-left: auto;
                margin-right: auto;
                margin-top: 60px;
            }
            li {
                color: white;
                margin: 0 auto;
                height: 50px;
                list-style-type: none;
            }
            footer {
                padding-bottom: 4rem;
                margin-top: 5rem;
                text-align: center;
                font-size: 14px;
            }
            footer li {
                padding-top: 25px;
                height: 10px;
            }
            .avatar {
                width: 100px;
                height: 100px;
                background-size: cover;
                border-radius: 9999px;
                margin: auto;
            }
            .title {
                padding-top: 1.5rem;
                width: 650px;
            }
            .summary {
                margin: auto;
                width: 650px;
                color: grey;
                line-height: 1.5rem;
                text-align: left;
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
            <footer><ul><li>{"Powered by Yew"}</li><li>{"粤ICP备2022118105号-1"}</li></ul></footer>
        </div>
    }
}
