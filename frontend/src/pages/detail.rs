use yew::{function_component, Html, html, Component, Context, Properties};


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct DetailProps{
    pub id: String,
}

#[function_component(Detail)]
pub fn detail(props: &DetailProps)->Html{


    html!{
        <div class="container">
            <h1 class="title">{"Item name"}</h1>
            <img class="image is-256x256 noscale" src="public/default.jpg" alt="defualt"/>
            <p>{"description"}</p>
            
            {props.id.clone()}
        </div>
    }
}