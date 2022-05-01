#![recursion_limit = "640"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::{Context, ContextProvider};
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
    Store,
    #[at("/:id")]
    Detail{id: i64},
    #[at("/profile")]
    Profile,
    #[at("/logout")]
    Logout,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/error")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
enum PublicRoute {
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn public_switch(route: &PublicRoute) -> Html {
    match route {
        PublicRoute::Login => html! {<login::LoginForm/>},
        PublicRoute::Signup => html! {<signup::SignupForm/>},
        PublicRoute::Contact => html! {<h2>{"Contact"}</h2>},
        PublicRoute::NotFound => html! {<Redirect<PublicRoute> to={PublicRoute::Login}/>},
    }
}

fn private_switch(route: &PrivateRoute) -> Html {
    match route {
        PrivateRoute::Store => html! {<store::Store/>},
        PrivateRoute::Detail{id} => html!{<detail::Detail id={*id}/>},
        PrivateRoute::Profile => html!{"Profile"},
        PrivateRoute::Contact => html!{"Contact"},
        PrivateRoute::Logout => {todo!()},
        PrivateRoute::NotFound => html! {<NotFound/>},
    }
}

#[function_component(App)]
fn app() -> Html {

    match GET_user_detail(){
        Ok(user) => {
        let ctx = use_state(|| models::user::UserInfo {
            ..user
        });

        html! {
            <>
                <ContextProvider<models::user::UserInfo> context={(*ctx).clone()}>
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
    },
    Err(_) => {
        return html!{
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

//#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
