use yew::prelude::*;
use alloy_primitives::Address;
use alloy_rpc_types::BlockNumberOrTag;
use std::{ops::Deref, str::FromStr};
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use crate::contexts::ethereum::UseEthereum;
use web_sys::HtmlInputElement;

/* 
TODO:
    - validate address
    - handle error cases

*/

#[function_component(GetCode)]
pub fn get_code() -> Html {
    
    let address = use_state(|| "Get code (insert valid address)".to_string());
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let client = ethereum.provider.clone();
     
    let get_code_address = {
        let addr = (*address).clone();
        let chain = ethereum.chain();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let addr = Address::from_str(&addr).expect("Cannot parse address");
                let client = client.clone();
                spawn_local(async move {
                    match client.get_code_at(addr, BlockNumberOrTag::Latest).await {
                        Ok(d) => log(format!("Code in {} address: {} is {:#?}", chain.unwrap(), &addr, d).as_str()),
                        Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                    }
                })
            } else { log("not connected") }
        })
    };
    
    let on_change_address = {
        let a = address.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            a.set(input.value());
        })
    };

    html!{
        <div class={"getCode"}>
            if ethereum.is_connected() {
                <input onchange={on_change_address} class={"address_input"} type="text" value={(*address).clone()} />
                if address.len() == 42 {
                    <button onclick={get_code_address} class="button">{"Get code"}</button>
                }
            }
        </div>
    }

}