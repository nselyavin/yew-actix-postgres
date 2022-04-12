use web_sys::console;
use yew::{html, use_context, Callback, Component, Context, Html};

use crate::models::item::Item;
use crate::utils::requests::*;

pub struct Store {
    items: Vec<Item>,
}

impl Component for Store {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut count = 0;
        let inited_items = Store::init_items();
        Self {
            items: inited_items,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let mut count = 0;
        let rows = self.items.len() / 4;
        let mut iter = self.items.iter();
        
        let row_vec = (0..=rows).collect::<Vec<_>>();
        let column_vec = (0..4).collect::<Vec<_>>();
        html! {
            <div class="store is-ancestor">
            {
                row_vec.iter().map(|ind|{
                    log::info!("{}", rows);
                    html!{
                        <div class="tile is-parent">
                        {
                            column_vec.iter().map(|_|{
                                let item = iter.next();
                                if let None = item {
                                    html!{}
                                } else {
                                    let item = item.unwrap();
                                    html!{
                                        <a class="store-tile scale tile is-3" href={format!("/{}", item.id)}>
                                        <article class="is-child box ">
                                            <p class="title noscale">{item.name.clone()}</p>
                                            <p class="subtitle noscale">{item.description.clone()}</p>
                                            <img class="image is-256x256 noscale" src="public/default.jpg"/>
                                        </article>
                                        </a>
                                    }
                                }
                            }).collect::<Html>()
                        }
                        </div>
                    }
                }).collect::<Html>()
            }
            </div>
        }


    }


        // <div class="store">
        //     {
        //         (1..=rows/4).iter().map(|ind|{
        //             self.items.iter().map(|it|{
        //                 html!{
        //                     <article class="tile is-child box">
        //                         <p class="title">{it.name.clone()}</p>
        //                         <p class="subtitle">{it.description.clone()}</p>
        //                         <img src="public/default.jpg" width="400" height="400"/>
                                    
        //                     </article>
        //                 }).collect::<Html>()
        //             }
        //         }).collect::<Html>()
        //     }
        //     </div>   
}

impl Store {
    fn init_items() -> Vec<Item>{
        match GET_items() {
            Ok(list) => {
                list
            },
            Err(val) => {
                log::info!("failed to get items: {}", val);
                Vec::default()
            },
        }
    }
}
