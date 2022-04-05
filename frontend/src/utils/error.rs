use yew::{function_component, html, Component, Context, Properties};

#[derive(Properties, PartialEq)]
pub struct ErrorProps {
    pub msg: Option<String>,
}

#[function_component(Error)]
pub fn error(props: &ErrorProps) -> Html {
    //let val = props.msg.as_ref().unwrap().clone();
    let value = props.msg.as_deref().unwrap_or("Unknown string");
    html!{
        <section  class="section">
            <h2 class="title">{"Happened some error"}</h2>
            <p class="subtitle">
                {value}
            </p>

            <button class="button is-primary">{"back"}</button>
        </section>
    }

}
