use yew::prelude::*;
use alloy_primitives::Address;
use alloy_json_abi::Event;
use std::{
    ops::BitXorAssign,
};
/* 
Create filter with event
Decode responses
how to make the component generic to inputs/outputs

*/
#[derive(Properties, PartialEq)]
pub struct Props {
    pub address: Address,
    pub event: Event,
//    pub placeholder: String,
}

#[function_component(EventComponent)]
pub fn event_component(props: &Props) -> Html {
    let open = use_state(|| false);   
    let error_msg = use_state(|| None as Option<String>);
        
    let toggle_comp = {
        let o = open.clone();
        let rv = (*o).clone();
        Callback::from(move |_e: MouseEvent| {
            let mut rvv = rv.clone();
            let _ = rvv.bitxor_assign(true);
            o.set(rvv);
        })
    };
    let button_label = match (*open).clone() {
        true => format!("Close {}", props.event.name),
        false => format!("{}",props.event.name),
    };

    let inputs_list = props.event.inputs
        .clone()
        .into_iter()
        .map(|v|  {
            html!(
                <table class={"function-table"}>
                    <tr>
                        <td>{format!("{} [{}]",v.name, v.ty)}</td>
                        //<td>{v.ty}</td>
                    </tr>
                </table>
            )
        }
        ).collect::<Html>();
    
    html! {
        <div>
            <button onclick={toggle_comp} class={"button-event"}>
                {button_label}
            </button>
            if (*open).clone() {
                <div class="contract_event">
                    <table class={"function-table"}> 
                        <tr>
                            <td>{"Name"}</td>
                            <td>{&props.event.name}</td>
                        </tr>
                        <tr>
                            <td>{"Inputs"}</td>
//                            <td>{format!("{:?}", props.event.inputs)}</td>
                            {inputs_list}
                        </tr>
                    </table>
                </div>
                if let Some(error) = (*error_msg).clone() {
                    <hr />
                    <small>{error}</small>
                }
            }
        </div>
    }
}