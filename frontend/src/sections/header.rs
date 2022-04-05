use yew::{function_component, html, Component, Context, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct HeaderProps {
    pub is_login: bool,
    pub username: Option<String>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <nav class="navbar">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">
                    <img src="logo.png" width="120" height="30"/>
                </a>
            </div>
            <div id="navbarBasicExample" class="navbar-menu">
                <div class="navbar-start">
                <a class="navbar-item" href="/">
                    {"Store"}
                </a>
                
                
        {
            if !props.is_login{
                html!{<div class="navbar-end">
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
            } else {
                html!{
                    <div class="navbar-item has-dropdown is-hoverable">
                    <a class="navbar-link">
                    {"Username"}
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
        </div>
    </div>
        </nav>
    }
}
