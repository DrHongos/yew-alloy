use yew::prelude::*;
use alloy_primitives::Address;
use alloy_rpc_types::BlockNumberOrTag;
use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use crate::contexts::ethereum::UseEthereum;
use crate::components::address_input::AddressInput;

/* 
TODO:
    - handle error cases

*/

#[function_component(GetCode)]
pub fn get_code() -> Html {    
    let address = use_state(|| None as Option<Address>);
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let client = ethereum.provider.clone();
     
    let get_code_address = {
        let addr = (*address).clone();
        let chain = ethereum.chain();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                if let Some(addr) = addr {
                    let client = client.clone();
                    spawn_local(async move {
                        match client.get_code_at(addr, BlockNumberOrTag::Latest).await {
                            Ok(d) => log(format!("Code in {} address: {} is {:#?}", chain.unwrap(), &addr, d).as_str()),
                            Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                        }
                    })
                }
            } else { log("not connected") }
        })
    };

    let on_add_address: Callback<Address> = {
        let a = address.clone();
        Callback::from(move |addr| {
            a.set(Some(addr));
        })
    };

    html!{
        <div class={"getCode"}>
            if ethereum.is_connected() {
                <AddressInput 
                    on_add={on_add_address}
                    show_me={false}
                    placeholder={String::from("Insert contract address")}
                />
                if (*address).clone().is_some() {
                    <button onclick={get_code_address} class="button">{"Get code"}</button>
                }
            }
        </div>
    }

}