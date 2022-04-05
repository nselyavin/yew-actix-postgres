#![recursion_limit = "640"]
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::prelude::*;

mod sections;
mod utils;
mod pages;
use pages::*;
use utils::error::Error;

#[derive(Clone, Routable, PartialEq)]
pub enum Route{
    #[at("/")]
    Store,
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

fn switch(route: &Route)->Html{
    match route {
        Route::Store => html!{"Store"},
        Route::Login => html!{<login::LoginForm/>},
        Route::Signup => html!{"Signup"},
        Route::Profile => html!{"Profile"},
        Route::Contact => html!{"Contact"},
        Route::Logout => {todo!()},
        Route::NotFound => html!{<Error msg={"pizda"}/>},
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <sections::header::Header is_login={false} />
                <main>
                    <BrowserRouter>
                        <Switch<Route> render={Switch::render(switch)} />
                    </BrowserRouter>
                </main>
                <sections::footer::Footer/>
            </>
        }
    }
}

//#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}