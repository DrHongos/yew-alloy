use yew::prelude::*;
use alloy_primitives::Address;
use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use crate::contexts::ethereum::UseEthereum;
use web_sys::HtmlInputElement;

/* 
TODO:
    - add blockId selector
    - validate address
    - handle error cases

*/

#[function_component(GetBalance)]
pub fn get_balance() -> Html {
    
    let address = use_state(|| "insert address to get balance".to_string());
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let client = ethereum.provider.clone();
    let main_account = ethereum.account();

    let get_balance = {
        let client = client.clone();
        let a = address.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let account: Address = a.parse().expect("address is wrong");
                let client = client.clone();
                    spawn_local(async move {    
                        match client.get_balance(account, None).await {
                            Ok(b) => log(format!("Balance of {}: {}", account, b).as_str()),
                            Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                        }
                    })
                } else { log("disconnected") }
    })};

    let on_change_address = {
        let a = address.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            a.set(input.value());
        })
    };
    let me = {
        let a = address.clone();
        Callback::from(move |_| {
            a.set(main_account.to_string());
        })
    };

    html!{
        <div class={"getCode"}>
            if ethereum.is_connected() {
                <input onchange={on_change_address} class={"address_input"} type="text" value={(*address).clone()} />
                <button onclick={me} class={"button"}>{"me"}</button>
                if address.len() == 42 {
                    <button onclick={get_balance} class="button">{"Get balance"}</button>
                }
            }
        </div>
    }

}