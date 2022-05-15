use yew::{function_component, Html, html, Component, Context, Properties, use_effect_with_deps};
use yew_hooks::{use_async, use_mount};
use yew_router::{history, hooks::use_history};

use crate::{models::medicines::{self, MedicineInfo}, utils::requests::request_get};

use super::profile;


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct DetailProps{
    pub id: String,
}

#[function_component(Info)]
pub fn info(props: &DetailProps)->Html{
    let medicine_info = {
        let url = format!("/info/{}", props.id.clone());
        use_async(async move{
            request_get::<MedicineInfo>(url).await
        })
    };

    {
        let medicine_info = medicine_info.clone();
        use_mount(move || {
            medicine_info.run();
        })
    }

    match &medicine_info.data{
        Some(medicine) => {
            html!{
                <div class="container medicine-info">
                    <h1 class="title">{medicine.name.clone()}</h1>
                    <img class="image is-256x256 noscale" src="/public/default.jpg" width=256 height=256 alt="defualt"/><br/>
                    <h4 class="title is-4">{"Parameters"}</h4>
                    <table class="table">
                        //<tbody>
                            <tr>
                                <th>{"Cost:"}</th>
                                <td>{medicine.cost}</td>
                            </tr>
                            <tr>
                                <th>{"Description:"}</th>
                                <td>{medicine.description.clone()}</td>
                            </tr>
                            <tr>
                                <th>{"Creator:"}</th>
                                <td>{medicine.creator_name.clone()}</td>
                            </tr>
                       // </tbody>
                    </table>
                    <br/>
                    {props.id.clone()}
                </div>
            }
        },
        None => {
            if let Some(_) = &medicine_info.error{
                html!{
                    <div class="section medicine-info">
                        <article class="message is-danger">
                            <div class="message-header">
                                <p>{"Danger"}</p>
                            </div>
                            <div class="message-body">
                                {format!("Pharmacy with id \'{}\' not found", props.id.clone())}
                            </div>
                    </article>
                    </div>
                }
            } else {
                html!{}
            }

        },
    }
}