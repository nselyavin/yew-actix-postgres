use yew::{function_component, NodeRef, html, use_context, Callback};

use crate::models;




#[function_component(NewMedicine)]
pub fn new_medicine()-> Html{
    let user_info = use_context::<models::user::UserInfo>();
    let name = NodeRef::default();
    let cost = NodeRef::default();
    let descr = NodeRef::default();
    
    let onsubmit = Callback::from(|_|{
        
    });

    html!{
        <div class="section new-medicine">
            <h2 class="title">{"New medicine"}</h2>
            <div class="field">
                <label class="label">{"Name"}</label>
                <div class="control">
                    <input class="input" ref={name.clone()} type="text" placeholder="Text input"/>
                </div>
            </div>
            
            <div class="field">
                <label class="label">{"Cost"}</label>
                <div class="control has-icons-right">
                <input class="input is-info" ref={cost.clone()} type="number" placeholder="Cost" value="0.0"/>
                <span class="icon is-small is-right">
                    <i class="fas fa-rub"></i>
                </span>
                </div>
            
                <div class="field">
                    <br/>
                    <label class="label">{"Message"}</label>
                    <div class="control">
                        <textarea class="textarea" ref={descr.clone()} placeholder="Textarea" ></textarea>
                    </div>
                </div>
                
                <div class="control">
                    <button class="button is-success" onclick={onsubmit}>{"Create"}</button>
                </div>
            </div>

        </div>
    }

}