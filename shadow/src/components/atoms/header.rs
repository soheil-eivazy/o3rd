use yew::prelude::*;
use stylist::{yew::styled_component, Style, style};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub color: Color,
}

#[derive(PartialEq)]
pub enum Color {
    Normal,
    Okay,
    Error,
}

impl Color {
    fn to_string(&self) -> String {
        match self {
            Color::Normal => "normal".to_string(),
            Color::Okay => "ok".to_string(),
            Color::Error => "error".to_string(),
        }
    } 
}

#[styled_component(Header)]
pub fn header(props: &Props) -> Html {

    let sheet = style_sheet();

    html! {
        <div class={sheet}>
            <h1 class={props.color.to_string()}>{props.title.clone()}</h1>
        </div>
    }
}



fn style_sheet() -> Style {
    style! {
        r#"

        * {
            background-color: black;
        }

        .normal {
            color: white;
        }

        .ok {
            color: green;
        }

        .error {
            color: red;
        }

        "#
    }.unwrap()
}