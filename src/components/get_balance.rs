use yew::prelude::*;
use alloy_primitives::Address;
use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use crate::contexts::ethereum::UseEthereum;
use crate::components::address_input::AddressInput;
/* 
TODO:
    - add blockId selector
    - validate address
    - handle error cases

*/

#[function_component(GetBalance)]
pub fn get_balance() -> Html {
    
    let address = use_state(|| None as Option<Address>);
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let client = ethereum.provider.clone();

    let get_balance = {
        let client = client.clone();
        let account = address.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let client = client.clone();
                let account = account.clone();
                spawn_local(async move {
                    if let Some(account) = (*account).clone() {
                        match client.get_balance(account, None).await {
                            Ok(b) => log(format!("Balance of {}: {}", account, b).as_str()),
                            Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                        }
                    }
                })
                } else { log("disconnected") }
    })};

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
                    show_me={true}
                    placeholder={String::from("Insert address to get balance")}
                />
/* 
                <input onchange={on_change_address} class={"address_input"} type="text" value={(*address).clone()} />
                <button onclick={me} class={"button"}>{"me"}</button>
                 */
                if (*address).clone().is_some() {
                    <button 
                        onclick={get_balance} 
                        class="button"
                        disabled={(*address).clone().is_none()}
                    >{"Get balance"}</button>
                }
            }
        </div>
    }

}