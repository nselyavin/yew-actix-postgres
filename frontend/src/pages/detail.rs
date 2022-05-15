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
            <img src="" alt="detail"/>
            <p>{"description"}</p>
            
            {props.id.clone()}
        </div>
    }
}