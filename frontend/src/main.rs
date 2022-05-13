#![recursion_limit = "640"]
use async_std::task::current;
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
    #[at("/:id")]
    Detail { id: i64 },
    #[at("/profile")]
    Profile,
    #[at("/logout")]
    Logout,
    #[not_found]
    #[at("/error")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
enum PublicRoute {
    #[at("/")]
    Search,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn public_switch(route: &PublicRoute) -> Html {
    match route {
        PublicRoute::Search => html! {},
        PublicRoute::Login => html! {<login::LoginForm/>},
        PublicRoute::Signup => html! {<signup::SignupForm/>},
        PublicRoute::NotFound => html! {<Redirect<PublicRoute> to={PublicRoute::Login}/>},
    }
}

fn private_switch(route: &PrivateRoute) -> Html {
    match route {
        PrivateRoute::Detail { id } => html! {<detail::Detail id={*id}/>},
        PrivateRoute::Profile => html! {"Profile"},
        PrivateRoute::Logout => {
            remove_token();
            html!{<Redirect<PublicRoute> to={PublicRoute::Login}/>}
        }
        PrivateRoute::NotFound => html! {<NotFound/>},
    }
}

#[function_component(App)]
fn app() -> Html {
    let user_ctx = use_state(|| Some(UserInfo::default()));
    let current_user = use_async(async move { request_get::<UserInfo>("/user/detail").await });

    {
        let current_user = current_user.clone();

        use_mount(move || {
            if get_token().is_some(){
                current_user.run();
            }
        })
    }

    {
        let user_ctx = user_ctx.clone();

        use_effect_with_deps(
            move |current_user| {
                match &current_user.data {
                    Some(user_info) => user_ctx.set(Some(user_info.clone())),
                    None => user_ctx.set(None),
                }

                || ()
            },
            current_user,
        )
    }

    {
        let user_ctx = user_ctx.clone();
        match &*user_ctx {
            Some(user_info) => {
                html! {
                    <>
                        <ContextProvider<models::user::UserInfo> context={(*user_info).clone()}>
                            <sections::header::Header />
                            <main>
                                <BrowserRouter>
                                    <Switch<PrivateRoute> render={Switch::render(private_switch)} />
                                </BrowserRouter>
                            </main>
                            <sections::footer::Footer/>
                        </ContextProvider<models::user::UserInfo>>
                    </>
                }
            }
            None => {
                return html! {
                    <>
                        <sections::header::Header />
                                <main>
                                    <BrowserRouter>
                                        <Switch<PublicRoute> render={Switch::render(public_switch)} />
                                    </BrowserRouter>
                                </main>
                        <sections::footer::Footer/>
                    </>
                }
            }
        }
    }
}

//#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
