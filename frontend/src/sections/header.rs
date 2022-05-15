use log::debug;
use yew::{function_component, html, use_context, Callback, Component, Context, Properties, classes, use_state, use_effect_with_deps, callback};

use crate::models::{self, user::UserInfo};

#[function_component(Header)]
pub fn header() -> Html {
    let opt_user = use_context::<models::user::UserInfo>();
    let nav_classes = use_state(||Some("navbar-menu"));

    let onclick = {
        let nav_classes = nav_classes.clone();
        
        Callback::from(move |_|{
            log::info!("Click");
            match *nav_classes {
                Some(_) => nav_classes.set(None),
                None => nav_classes.set(Some("is-active")),
            }
        })
    };

    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">
                    <img src="public/logo.png" width="120" height="30"/>
                </a>
                <a role="button" class="navbar-burger" {onclick} aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>
            <div id="navbarBasic" class={classes!("navbar-menu", *nav_classes)}>
                <div class="navbar-item">
                    <a class="navbar-item"  href="/">
                        {"Search"}
                    </a>
                </div>
                    
            {
            match opt_user{
                None => {
                    html!{
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
        </nav>
    }
}
