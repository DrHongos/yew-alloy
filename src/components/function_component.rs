use yew::prelude::*;
use alloy_primitives::{Address, Bytes};
use alloy_json_abi::{Function, StateMutability};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlButtonElement;
use crate::contexts::ethereum::UseEthereum;
use std::{
    ops::BitXorAssign,
    str::FromStr,
};
use alloy_rpc_types::{CallRequest, CallInput};
/* 
Add inputs to call
Decode responses
Store last response timestamped (type is function dependent)
how to make the component generic to inputs/outputs

*/
#[derive(Properties, PartialEq)]
pub struct Props {
    pub address: Address,
    pub function: Function,
//    pub placeholder: String,
}

#[function_component(FunctionComponent)]
pub fn function_component(props: &Props) -> Html {
    let open = use_state(|| false);   
    let error_msg = use_state(|| None as Option<String>);
    
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let provider = ethereum.provider.clone();
    
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
        true => format!("Close {}", props.function.name),
        false => format!("{}",props.function.name),
    };

    let call_function = {
        let provider = provider.clone();
        let address = props.address.clone();
        Callback::from(move |e: MouseEvent| {
            let t: HtmlButtonElement = e.target_unchecked_into();
            let fn_selector = t.value();
            let tx = CallRequest {
                to: Some(address),
                input: CallInput::new(Bytes::from_str(&fn_selector).expect("Error parsing bytes")),
                from: None,
                gas_price: None,
                max_fee_per_gas: None,
                max_priority_fee_per_gas: None,
                gas: None,
                value: None,
                nonce: None,
                chain_id: None,
                access_list: None,
                max_fee_per_blob_gas: None,
                blob_versioned_hashes: None,
                transaction_type: None,
            };
            let provider = provider.clone();
            spawn_local(async move {
                crate::helpers::log(format!("tx is {:#?}", tx).as_str());
                if let Some(provider) = (*provider).clone() {
                    let c = provider.call(tx, None);
                    match c.await {
                        Ok(bn) => crate::helpers::log(format!("Call result: {}", bn).as_str()),
                        Err(rv) => crate::helpers::log(format!("Error: {:#?}", rv).as_str())
                    }
                }
            })
        })
    };

    let inputs_list = props.function.inputs
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

    let outputs_list = props.function.outputs
        .clone()
        .into_iter()
        .map(|v|  {
            html!(
                <table class={"function-table"}>
                    <tr>
                        <td>{v.name}</td>
                        <td>{v.ty}</td> 
                        // input?
                    </tr>
                </table>
            )
        }
        ).collect::<Html>();
    
    html! {
        <div>
            <button onclick={toggle_comp} class={"button"}>
                {button_label}
            </button>
            if (*open).clone() {
                <div class="contract_function">
                    <table class={"function-table"}> 
                        <tr>
                            <td>{"Name"}</td>
                            <td>{&props.function.name}</td>
                        </tr>
                        <tr>
                            <td>{"State Mutability"}</td>
                            <td>{format!("{:?}", props.function.state_mutability)}</td>
                        </tr>
                        <tr>
                            <td>{"Inputs"}</td>
                            {inputs_list}
                        </tr>
                        <tr>
                            <td>{"Outputs"}</td>
                            {outputs_list}
                        </tr>
                    </table>
                    if props.function.inputs.len() == 0 && props.function.state_mutability != StateMutability::Payable {
                        <button onclick={call_function} value={props.function.selector().to_string()} class={"execute-button"}>{"Call"}</button>
                    }
                </div>
                if let Some(error) = (*error_msg).clone() {
                    <hr />
                    <small>{error}</small>
                }
            }
        </div>
    }
}