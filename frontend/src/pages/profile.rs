
use yew::{function_component, Html, html, Component, Context, Properties};
use yew_hooks::use_mount;
use yew_router::{hooks::use_history, history::History};

use crate::{utils::requests::get_token, PrivateRoute};


#[function_component(Profile)]
pub fn profile()->Html{

    //use_mount(||{
        if get_token().is_none(){
            use_history().unwrap().push(PrivateRoute::Login);
        }
    //});

    html!{
        <div class="container">
            {"Profile"}
        </div>
    }
}