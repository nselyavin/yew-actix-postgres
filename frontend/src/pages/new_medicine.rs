use yew::{function_component, NodeRef, html, use_context};

use crate::models;




#[function_component(NewMedicine)]
pub fn new_medicine()-> Html{
    let user_info = use_context::<models::user::UserInfo>();
    let name = NodeRef::default();
    let cost = NodeRef::default();
    

    html!{
        <div class="section">
            

        </div>
    }

}