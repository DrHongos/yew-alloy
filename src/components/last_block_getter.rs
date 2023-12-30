use yew::prelude::*;
use crate::contexts::ethereum::UseEthereum;
use web_sys::HtmlInputElement;
use alloy_rpc_types::{Block, BlockNumberOrTag};
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use std::ops::Deref;
/* 
TODO:
    - handle error cases
    - display info
    - add checkbox for complete data request
*/

#[function_component(LastBlockGetter)]
pub fn last_block_getter() -> Html {
    let complete = use_state(|| false);
    //let block = use_state(|| None::<Block>);
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let client = ethereum.provider.clone();
    
    let get_last_block = {
        let client = client.clone();
        Callback::from(move |_: MouseEvent| {
            let client = client.clone();
            let complete = (*complete).clone();
            spawn_local(async move {
                if let Some(client) = client.deref() {
                    match client.get_block_by_number(BlockNumberOrTag::Latest, complete).await {
                        Ok(d) => log(format!("latest block: is {:#?}", d).as_str()),
                        _ => log("Error!")
                    }     
                } else { log("not connected") }
            })
            
        })
    };
    
    html!{
        <div class={"getCode"}>
            if ethereum.is_connected() {
                <button onclick={get_last_block} class="button">{"Get last block"}</button>
                // add checkbox para "complete"
                
            }
        </div>
    }

}