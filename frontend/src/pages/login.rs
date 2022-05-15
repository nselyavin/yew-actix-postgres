use log;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, InputEvent, MouseEvent};
use yew::{function_component, use_effect_with_deps, use_mut_ref, use_state};
use yew::{html, Callback};
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::models::user::{UserLogin, UserToken};
use crate::utils::requests::{request_post, set_token};
use crate::PrivateRoute;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let data_state = use_mut_ref(|| UserLogin::default());
    let error = use_state::<Option<String>, _>(|| None);
    let user_login = use_async({
        let data_state = data_state.clone();
        async move {
            log::info!("data_state: {:?}", &*data_state.borrow_mut());
            request_post::<UserLogin, UserToken>("/login".to_string(), &*data_state.borrow_mut())
                .await
        }
    });

    let history = use_history().unwrap();
    {
        let error = error.clone();
        use_effect_with_deps(
            move |user_login| {
                if let Some(user_token) = &user_login.data {
                    set_token(user_token.token.clone());
                    log::info!("history push");
                    history.push(PrivateRoute::Profile);
                }

                if let Some(_) = &user_login.error {
                    log::info!("Failed to auth");
                    error.set(Some("Failed to auth".to_string()));
                }
                || ()
            },
            user_login.clone(),
        );
    }

    let change_email = {
        let data_state = data_state.clone();

        Callback::from(move |event: InputEvent| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let mut data = (*data_state).clone();
            (*data.borrow_mut()).email = input.unwrap().value();
            *data_state.borrow_mut() = (*data.borrow_mut()).clone();
        })
    };

    let change_password = {
        let data_state = data_state.clone();

        Callback::from(move |event: InputEvent| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let mut data = (*data_state).clone();
            (*data.borrow_mut()).password = input.unwrap().value();
            *data_state.borrow_mut() = (*data.borrow_mut()).clone();
        })
    };

    let onclick = {
        let data_state = data_state.clone();

        Callback::once(move |e: MouseEvent| {
            if (*data_state.borrow_mut()).is_empty() {
                return;
            }
            user_login.run();
        })
    };

    html! {
        <div class="login-form section">
        {
            if let Some(msg) = &*error{
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
                        <input class="input" name="email" type="email" oninput={change_email} placeholder="Email"/>
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
                        <input class="input" name="password" type="password" oninput={change_password} placeholder="Password"/>
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
