use bcrypt::{hash, verify, DEFAULT_COST};
use log;
use serde::ser::Error;
use serde::{Deserialize, Serialize};
use validator::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Callback, Component, Context, NodeRef};
use yew_router::prelude::*;

use crate::PublicRoute;
use crate::models::user::User;
use crate::utils::error_to_str::*;

pub enum SignupMessage {
    Signup,
    UpdateField,
}

enum ErrorType {
    UnknowUser,
    BadEmail,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct SignupData {
    #[validate(email)]
    email: String,
    username: String,
    #[validate(length(min = 8))]
    password: String,
}

impl SignupData{
    pub fn new()->SignupData{
        SignupData { 
            email: String::default(), 
            username: String::default(), 
            password: String::default() }
    }

    pub fn is_empty(&self)->bool{
        !(self.email.len() > 0 && self.username.len() > 0 && self.password.len() > 0)
    }
}

pub struct SignupForm {
    pub is_auth: bool,
    data: SignupData,
    error: Option<String>,
    email: NodeRef,
    username: NodeRef,
    password: NodeRef,
}

impl SignupForm {
    fn update_field(&mut self, field: &str, target: &Event){
        let value = target
                .target()
                .expect("Event should have a target when dispatched")
                .unchecked_into::<HtmlInputElement>().value();

        match field {
            "email" => self.data.email = value.clone(),
            "username" => self.data.email = value.clone(),
            _ => (),
        };
    }
}


impl Component for SignupForm {
    type Message = SignupMessage;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            is_auth: false,
            data: SignupData::new(),
            error: None,
            email: NodeRef::default(),
            username: NodeRef::default(),
            password: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        // On login
        let onclick = ctx.link().callback(|_| SignupMessage::Signup);
        html! {
            <div class="login-form section">

            {
                if let Some(msg) = &self.error{
                    html!{
                        <div class="notification is-danger">
                            <p>{format!("Error: {}", msg.clone())}</p>
                        </div>
                    }
                } else {
                    html!{}
                }
            }


            <h2 class="title">{"Login"}</h2>
            <div class="field">
                <p class="control has-icons-left has-icons-right">
                        <input class="input" type="email" ref={self.email.clone()} onchange ={ctx.link().callback(|_|{SignupMessage::UpdateField})} placeholder="Email"/>
                    <span class="icon is-small is-left">
                        <i class="fas fa-envelope"></i>
                    </span>
                </p>
            </div>
            <div class="field">
                <p class="control has-icons-left has-icons-right">
                    <input class="input" type="text" ref={self.username.clone()} onchange ={ctx.link().callback(|_|{SignupMessage::UpdateField})}  placeholder="Username"/>
                    <span class="icon is-small is-left">
                        <i class="fas fa-user"></i>
                    </span>
                </p>
            </div>
            <div class="field">
                <p class="control has-icons-left">
                    <input class="input" type="password" ref={self.password.clone()} onchange ={ctx.link().callback(|_|{SignupMessage::UpdateField})} placeholder="Password"/>
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SignupMessage::Signup => {
                log::info!("Signup: {:?}", self.data);

                if self.data.is_empty(){
                    self.error = Some("Fill all fields".to_string());
                    return true;
                }

                if let None = self.error{
                    let history = ctx.link().history().unwrap();
                    history.push(PublicRoute::Login);
                }
                true
            }
            SignupMessage::UpdateField => {
                self.data.email = self.email.cast::<HtmlInputElement>().unwrap().value();
                self.data.username = self.username.cast::<HtmlInputElement>().unwrap().value();
                self.data.password = self.password.cast::<HtmlInputElement>().unwrap().value();
                self.error = None;

                let res = self.data.validate();
                if let Err(errs) = res{
                    self.error = Some(validErr_to_str(&errs));
                }

                false
            }

        }
    }
}
