
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Mutex, Arc};

use bcrypt::{hash, verify, DEFAULT_COST};
use log;
use serde::{Deserialize, Serialize};
use validator::*; //{Validate, ValidateArgs, ValidationError, ValidationErrors};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Callback, Component, Context, NodeRef};
use yew::{use_state, UseStateHandle, function_component};
use yew_router::prelude::*;

use crate::models::user::{UserLogin, UserToken};
use crate::PrivateRoute;
use crate::utils::error_to_str::validErr_to_str;
use crate::utils::requests::{request_post, set_token};


#[function_component(LoginForm)]
pub fn login_form() -> Html{
    let data_state = use_state(|| UserLogin::default());

    let history = use_history().unwrap();
    let onclick = {
        let data_state = data_state.clone();
        
        Callback::once(move |_|{
            if data_state.is_empty(){
                return;
            }

            let data_state = data_state.clone();
            wasm_bindgen_futures::spawn_local(async move{
                let res = request_post::<UserLogin, UserToken>("/login", &data_state).await;
                
                log::info!("Token: {:?}", res);

                if let Ok(token) = res{
                    set_token(token.token);
                    history.push(PrivateRoute::Profile);
                } 
            });
        })
    };


    let onchange_email = {
        let data_state = data_state.clone();

        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let mut data = (*data_state).clone();
            data.email = input.unwrap().value();
            data_state.set(data);
        })
    };

    let onchange_password =  {
        let data_state = data_state.clone();

        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let mut data = (*data_state).clone();
            data.password = input.unwrap().value();

            data_state.set(data);
        })
    };

    html! {
        <div class="login-form section">
            <h2 class="title">{"Login"}</h2>
            <div class="field">
                    <p class="control has-icons-left has-icons-right">
                        <input class="input" type="email" onchange={onchange_email} placeholder="Email"/>
                        <span class="icon is-small is-left">
                        <i class="fas fa-envelope"></i>
                        </span>
                        <span class="icon is-small is-right">
                        <i class="fas fa-check"></i>
                        </span>
                    </p>
                    </div>
                    <div class="field">
                    <p class="control has-icons-left">
                        <input class="input" type="password" onchange={onchange_password} placeholder="Password"/>
                        <span class="icon is-small is-left">
                        <i class="fas fa-lock"></i>
                        </span>
                    </p>
                    </div>
                    <div class="field">
                    <p class="control">
                        <button class="button is-success" {onclick}>
                        {"Login"}
                        </button>
                    </p>
                </div>
        </div>
    }
}