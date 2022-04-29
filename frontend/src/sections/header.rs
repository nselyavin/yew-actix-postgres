use log::debug;
use yew::{function_component, html, Component, Context, Properties, use_context, Callback};

use crate::models::{user::User, self};


#[function_component(Header)]
pub fn header() -> Html {

    let opt_user = use_context::<models::user::User>();

    html! {
        <nav class="navbar">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">
                    <img src="public/logo.png" width="120" height="30"/>
                </a>
            </div>
            <div id="navbarBasicExample" class="navbar-menu">
                <div class="navbar-start">
                <a class="navbar-item" href="/">
                    {"Store"}
                </a>
                
                
        {
            match opt_user{
                None => {
                    html!{
                        <div class="navbar-end">
                            <div class="navbar-item">
                                <div class="buttons">
                                    <a class="button is-primary" href="/signup">
                                        <strong>{"Sign up"}</strong>
                                    </a>
                                    <a class="button is-light" href="/login">
                                    {"Log in"}
                                    </a>
                                </div>
                            </div>
                        </div>
                    }
                },
                Some(user) => {
                    html!{
                        <div class="navbar-item has-dropdown is-hoverable">
                            <a class="navbar-link">
                                {user.username.clone()}
                            </a>
                                
                            <div class="navbar-dropdown">
                                <a class="navbar-item" href="/profile">
                                    {"Profile"}
                                </a>
                                <a class="navbar-item" href="/contact">
                                    {"Contact"}
                                </a>
                                <hr class="navbar-divider"/>
                                <a class="navbar-item" href="/logout">
                                    {"Logout"}
                                </a>
                            </div>
                        </div>
                    }
                }
            }
        }
            </div>
         </div>
        </nav>
    }
}
