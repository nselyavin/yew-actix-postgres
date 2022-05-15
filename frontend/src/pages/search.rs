use async_std::future;
use gloo_utils::history;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{function_component, events::Event, html, Callback, use_state};
use yew_hooks::{use_async_with_options, use_async};
use yew_router::{hooks::use_history, history::History};

use crate::{PublicRoute, PrivateRoute};

#[function_component(Search)]
pub fn search() -> Html {
    let pharm_id = use_state(|| String::default());
    let history = use_history().unwrap();

    let onclick = Callback::once(
        move |_| {    
            history.push(PrivateRoute::Detail { id: "asd".to_string() })
            
        }
    );

    let onchange = {
        let pharm_id = pharm_id.clone();        

        Callback::from(move |event: Event|{
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            pharm_id.set(input.unwrap().value());

            log::info!("pharm id: {:?}", *pharm_id);
        })
    };

    html! {

        <div class="section search-form">
            <h2 class="title">{"Pharmacy search"}</h2>
            <div class="notification is-primary">
            <h2>{"Enter ID of your pharmacy product"}</h2>
            <div class="field has-addons">
                <div class="control is-expanded">
                    <input class="input" type="text" {onchange} placeholder="pharmacy id"/>
                </div>
                <div class="control">
                    <button class="button is-info" {onclick}>
                        {"Search"}
                    </button>
                </div>
            </div>
            </div>
        </div>
    }
}
