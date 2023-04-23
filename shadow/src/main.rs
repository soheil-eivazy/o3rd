use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component, style, css, Style, StyleSource};
use yew::prelude::*;
use gloo::console::log;
use serde_json;

mod components;

use components::atoms::header::{Header, Color};

fn main() {
    yew::Renderer::<App>::new().render();
}


#[derive(Serialize, Deserialize)]
struct ToBeLogged {
    title: String,
    text: String,
}


#[styled_component(App)]
fn app() -> Html {
    log!("will I be shown?");

    let logging = ToBeLogged{title: "new struct".to_owned(), text: "will be shown".to_string()};
    log!(serde_json::to_string_pretty(&logging).unwrap());

    let data = vec![
        "something",
        "maybe",
        "no not this one",
        "yes this",
    ];



    let style_class = style!{
        r#"
            color: red;
        "#
    }.unwrap();


    html! {
        <div>
        <Header title="this is the header" color={Color::Okay}/>
            <p class={style_class}>{"things go after this:"}</p>
            <ul>
                {list_parser(data)}
            </ul>
        </div>
    }
}


fn list_parser(data: Vec<&str>) -> Html {
    data.iter().map(|a| html! { <li class={css!{"color: green;"}}>{a}</li>}).collect()
}