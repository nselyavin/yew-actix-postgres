#![recursion_limit = "640"]
use async_std::task::current;
use gloo_utils::history;
use models::user::{self, UserInfo};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::{Context, ContextProvider};
use yew_hooks::{use_async, use_mount};
use yew_router::prelude::*;

mod models;
mod pages;
mod sections;
mod utils;
use pages::*;
use utils::not_found::NotFound;
use utils::requests::*;

#[derive(Clone, Routable, PartialEq)]
pub enum PrivateRoute {
    #[at("/")]
    Search,
    #[at("/info/:id")]
    Detail { id: String },
    #[at("/profile")]
    Profile,
    #[at("/medicine/new")]
    NewMedicine,
    #[not_found]
    #[at("/error")]
    NotFound,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
}

fn private_switch(route: &PrivateRoute) -> Html {
    match route {
        PrivateRoute::Detail { id } => html! {<info::Info id={(*id).clone()}/>},
        PrivateRoute::Profile => html! {<profile::Profile/>},
        PrivateRoute::NotFound => html! {<NotFound/>},
        PrivateRoute::Search => html! {<search::Search/>},
        PrivateRoute::NewMedicine => html! {<new_medicine::NewMedicine/>},
        PrivateRoute::Login => html! {<login::LoginForm/>},
        PrivateRoute::Signup => html! {<signup::SignupForm/>},
    }
}

#[function_component(App)]
fn app() -> Html {
    let user_ctx = use_state(|| Some(UserInfo::default()));
    let login_ctx = use_state(|| false);
    let current_user =
        use_async(async move { request_get::<UserInfo>("/user/detail".to_string()).await });

    {
        let current_user = current_user.clone();

        use_mount(move || {
            if get_token().is_some() {
                current_user.run();
            }
        })
    }

    {
        let user_ctx = user_ctx.clone();

        use_effect_with_deps(
            move |current_user| {
                if let Some(user_info) = &current_user.data {
                    user_ctx.set(Some(user_info.clone()))
                }
                if let Some(_) = &current_user.error{
                    user_ctx.set(None);
                    remove_token();
                }

                || ()
            },
            current_user,
        )
    }

    {
        let user_ctx = user_ctx.clone();
        html! {
            <>
                <ContextProvider<Option<UserInfo>> context={(*user_ctx).clone()}>
                    <sections::header::Header />
                    <main>
                        <BrowserRouter>
                            <Switch<PrivateRoute> render={Switch::render(private_switch)} />
                        </BrowserRouter>
                    </main>
                    <sections::footer::Footer/>
                </ContextProvider<Option<UserInfo>>>
            </>
        }
    }
}

//#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
