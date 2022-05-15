use async_std::future;
use gloo_utils::history;
use yew::{function_component, html, Callback, use_state};
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


    html! {
        <div class="section">
            <h2 class="title">{"Pharmacy search"}</h2>
            <div class="notification is-primary">
            <h2>{"Enter ID of your pharmacy product"}</h2>
            <div class="field has-addons">
                <div class="control is-expanded">
                    <input class="input" type="text" placeholder="pharmacy id"/>
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
