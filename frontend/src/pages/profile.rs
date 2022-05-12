
use yew::{function_component, Html, html, Component, Context, Properties};


#[function_component(Detail)]
pub fn profile()->Html{

    html!{
        <div class="container">
            <h1 class="title">{"Profile"}</h1>
            <img src="" alt="detail"/>
            <p>{"description"}</p>
            
        </div>
    }
}