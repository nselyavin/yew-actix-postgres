use yew::{function_component, Html, html, Component, Context, Properties};

use crate::utils::requests::*;
use crate::utils::image::Img;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct DetailProps{
    pub id: i64,
}

#[function_component(Detail)]
pub fn detail(props: &DetailProps)->Html{
    let item = GET_item(props.id).unwrap();
    
    html!{
        <div class="container">
            <h1 class="title">{item.name.clone()}</h1>
            <Img src="" alt="detail" width=256 height=256/>
            <p>{item.description.clone()}</p>
            <span>{"Components:"}</span>
            <ul>
            {
                item.components.iter().map(|x|{html!{
                    <li>{x.clone()}</li>
                }}).collect::<Html>()
            }
            </ul>            
            <a href="/" class="button is-primary">{"back"}</a>
        </div>
    }
}