use alloy_chains::Chain;
use web_sys::HtmlInputElement;
use crate::contexts::ethereum::UseEthereum;
use foundry_block_explorers::Client;
use yew::prelude::*;
//use wasm_bindgen_futures::spawn_local;
/* 
check client is valid
*/

#[function_component(EtherscanApiInput)]
pub fn etherscan_api_input() -> Html {
    let error_msg = use_state(|| None as Option<String>);
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let storage_key = "etherscan_api";
    let etherscan_client = ethereum.etherscan_client.clone();

    let on_api_key = {
        let chain = ethereum.chain().unwrap_or(Chain::mainnet());
        let err = error_msg.clone();
        Callback::from(move |e: Event| {
            let v: HtmlInputElement = e.target_unchecked_into();
            let v = v.value();
            let err = err.clone();
            match Client::new(chain, v.clone()) {
                Ok(_client) => { 
                    // should check the client!
                    if let Ok(Some(local_storage)) = web_sys::window().expect("No window?").local_storage() {
                        let _ = local_storage.set(storage_key, &v);
                    }
                 },
                Err(e) => { err.set(Some(e.to_string())) }
            }
        })
    };

    html! {
        <div>
            if (*etherscan_client).as_ref().is_none() {
                <div class="ea_input">
                    <input 
                        onchange={on_api_key} 
                        id={"ea_input"} 
                        class={"address_input"} 
                        placeholder={"Store Etherscan API key"}
                        type="text" 
                    />
                </div>
                if let Some(error) = (*error_msg).clone() {
                    <hr />
                    <small>{error}</small>
                }
            }
        </div>
    }
}