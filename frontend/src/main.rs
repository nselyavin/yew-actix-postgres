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

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Store,
    #[at("/:id")]
    Detail{id: String},
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
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

fn switch(route: &Route) -> Html {
    match route {
        Route::Store => html! {<store::Store/>},
        Route::Detail{id} => html!{<detail::Detail id={id.clone()}/>},
        Route::Login => html!{<login::LoginForm/>},
        Route::Signup => html!{"Signup"},
        Route::Profile => html!{"Profile"},
        Route::Contact => html!{"Contact"},
        Route::Logout => {todo!()},
        Route::NotFound => html! {<NotFound/>},
    }
}

#[function_component(App)]
fn app() -> Html {
    let ctx = use_state(|| models::user::User {
        email: "".to_owned(),
        username: "".to_owned(),
    });

    html! {
        <>
            <ContextProvider<models::user::User> context={(*ctx).clone()}>
                <sections::header::Header is_login={false}/>
                <main>
                    <BrowserRouter>
                        <Switch<Route> render={Switch::render(switch)} />
                    </BrowserRouter>
                </main>
                <sections::footer::Footer/>
            </ContextProvider<models::user::User>>
        </>
    }
}

//#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
