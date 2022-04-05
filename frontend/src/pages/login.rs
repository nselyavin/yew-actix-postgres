use bcrypt::{hash, verify, DEFAULT_COST};
use log;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidateArgs, ValidationError};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Callback, Component, Context, NodeRef};
use yew_router::prelude::*;

use crate::Route;

pub enum LoginMessage {
    Login,
    ChangeEmail(String),
    ChangePassword(String),
}

enum ErrorType {
    UnknowUser,
    BadEmail,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginData {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

impl LoginData {
    pub fn new() -> LoginData {
        LoginData {
            email: "".to_owned(),
            password: "".to_owned(),
        }
    }
}

pub struct LoginForm {
    pub is_auth: bool,
    data: LoginData,
}

impl Component for LoginForm {
    type Message = LoginMessage;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            is_auth: false,
            data: LoginData::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let onclick = ctx.link().callback_once(|_| LoginMessage::Login);
        let on_email_change = ctx.link().callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            LoginMessage::ChangeEmail(target.unchecked_into::<HtmlInputElement>().value())
        });

        let on_password_change = ctx.link().callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            LoginMessage::ChangePassword(target.unchecked_into::<HtmlInputElement>().value())
        });

        html! {
            <div class="login-form section">
            <h2 class="title">{"Login"}</h2>
            <div class="field">
                    <p class="control has-icons-left has-icons-right">
                        <input class="input" type="email" onchange={on_email_change} placeholder="Email"/>
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
                        <input class="input" type="password" onchange={on_password_change} placeholder="Password"/>
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
            LoginMessage::Login => {
                log::info!("Login");
                
                let history = ctx.link().history().unwrap();
                history.push(Route::Store);
                true
            }
            LoginMessage::ChangeEmail(val) => {
                // TODO: change mail
                log::info!("email: {}", val);
                self.data.email = val.clone();
                false
            }
            LoginMessage::ChangePassword(val) => {
                let hashed = hash("hunter2", 10).unwrap();
                log::info!("password: {:?}", hashed);
                //self.data.password = val.clone();
                false
            }
        }
    }
}
