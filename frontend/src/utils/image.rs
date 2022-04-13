use yew::{function_component, html, Component, Context, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ImageProps {
    pub src: Option<String>,
    pub alt: String,
    pub width: i32,
    pub height: i32,
}

#[function_component(Img)]
pub fn image(props: &ImageProps) -> Html {
    html! {
        <img 
        src={
            match &props.src{
                Some(val) => {
                    if(val.len() > 0){
                        val.clone()
                    } else {
                        "public/default.jpg".to_owned()
                    }
                },
                None => "public/default.jpg".to_owned(),
            }
        }
        width=256
        height=256
        alt={props.alt.clone()}
        />
    }
}
