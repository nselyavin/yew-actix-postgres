use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::{sync::{Arc, Mutex}, borrow::BorrowMut};
use bcrypt::{hash, verify, DEFAULT_COST};
use log;
use serde::ser::Error;
use serde::{Deserialize, Serialize};
use serde_json::error;
use validator::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{use_effect, Properties, function_component, use_state, use_mut_ref, use_effect_with_deps, UseStateHandle};
use yew::{events::Event, html, Callback, Component, Context, NodeRef};
use yew_router::prelude::*;

use crate::{PublicRoute, PrivateRoute};
use crate::models::user::UserSignup;
use crate::utils::{requests::request_post, error_to_str::validErr_to_str};


#[function_component(SignupForm)]
pub fn signup_form() -> Html{
    let data_state = use_state(|| UserSignup::default());
    let error: UseStateHandle<Option<String>> = use_state(|| None);

    let onclick = {
        let data_state = data_state.clone();
        let error = error.clone();
        
        Callback::from(move |_|{
            if data_state.is_empty(){
                error.set(Some("Fill all fields".to_string()));
                return;
            }

            let data_state = data_state.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move{
                let res = request_post::<UserSignup, ()>("/signup".to_string(), &data_state).await;
                
                if let Err(status) = res{
                    if status == 409{
                        error.set(Some("User already exists".to_string()));
                    } else {
                        error.set(Some("User successfuly created".to_string()));
                    }
                } 
            });
        })
    };


    let onchange_email = {
        let data_state = data_state.clone();
        let error = error.clone();

        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let mut data = (*data_state).clone();
            data.email = input.unwrap().value();

            error.set(None);
            let res = data.validate();
            if let Err(errs) = res{
                error.set(Some(validErr_to_str(&errs)));
            }
            data_state.set(data);
        })
    };

    let onchange_username =  {
        let data_state = data_state.clone();
        let error = error.clone();

        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let mut data = (*data_state).clone();
            data.username = input.unwrap().value();

            error.set(None);
            let res = data.validate();
            if let Err(errs) = res{
                error.set(Some(validErr_to_str(&errs)));
            }
            data_state.set(data);
        })
    };

    let onchange_password =  {
        let data_state = data_state.clone();
        let error = error.clone();

        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let mut data = (*data_state).clone();
            data.password = input.unwrap().value();

            error.set(None);
            let res = data.validate();
            if let Err(errs) = res{
                error.set(Some(validErr_to_str(&errs)));
            }
            data_state.set(data);
        })
    };

    html! {
        <div class="login-form section">

        {
            if let Some(msg) = (*error).clone(){
                html!{
                    <div class="notification is-danger">
                        <p>{format!("Error: {}", msg.clone())}</p>
                    </div>
                }
            } else {
                html!{}
            }
        }


            <h2 class="title">{"Signup"}</h2>
            <div class="field">
                <p class="control has-icons-left has-icons-right">
                        <input class="input" name="email" type="email" onchange={onchange_email} placeholder="Email"/>
                    <span class="icon is-small is-left">
                        <i class="fas fa-envelope"></i>
                    </span>
                </p>
            </div>
            <div class="field">
                <p class="control has-icons-left has-icons-right">
                    <input class="input" name="nickname" type="text" onchange={onchange_username}placeholder="Username"/>
                    <span class="icon is-small is-left">
                        <i class="fas fa-user"></i>
                    </span>
                </p>
            </div>
            <div class="field">
                <p class="control has-icons-left">
                    <input class="input" name="password" type="password" onchange={onchange_password} placeholder="Password"/>
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