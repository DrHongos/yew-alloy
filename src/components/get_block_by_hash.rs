use yew::prelude::*;
use crate::contexts::ethereum::UseEthereum;
use alloy_rpc_types::Block;
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use std::ops::{Deref, BitXorAssign};
use alloy_primitives::BlockHash;
use web_sys::HtmlInputElement;
/* 
TODO:
    - handle error cases
*/
#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_block: Callback<Block>,
}

#[function_component(GetBlockByHash)]
pub fn get_block_by_hash(props: &Props) -> Html {
    let full = use_state(|| false);
    let hash = use_state(|| None as Option<BlockHash>);
    let error_msg = use_state(|| None as Option<String>);
    //let block = use_state(|| None::<Block>);
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let client = ethereum.provider.clone();
    
    let get_block = {
        let client = client.clone();
        let pprops = props.on_block.clone();
        if let Some(hash) = (*hash).clone() {
            let full = full.clone();
            let pprops = pprops.clone();
            Callback::from(move |_: MouseEvent| {
                let client = client.clone();
                let hash = hash.clone();
                let full = (*full).clone();
                let pprops = pprops.clone();
                spawn_local(async move {
                    if let Some(client) = client.deref() {
                        match client.get_block_by_hash(hash.clone(), full).await {
                            Ok(d) => pprops.emit(d.expect("No block!")),
                            _ => log("Error!")
                        }     
                    } else { log("not connected") }
                })
                
            })
        } else {
            Callback::from(|_| {log("Select a block")})
        }
    };

    let on_change_hash = {
        let h = hash.clone();
        let err = error_msg.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            match input.value().parse::<BlockHash>() {
                Ok(a) => {
                    err.set(None);
                    h.set(Some(a));
                },
                Err(e) => err.set(Some(e.to_string()))
            }
        })
    };
    let on_full_data_checkbox = {
        let c = full.clone();
        let rv = (*c).clone();
        Callback::from(move |_e: Event| {
            let mut rvv = rv.clone();
            let _ = rvv.bitxor_assign(true);
            c.set(rvv);
        })
    };

    html!{
        <div class={"getCode"}>
            if ethereum.is_connected() {
                <input 
                    onchange={on_change_hash} 
                    id={"hash_input"} 
                    class={"address_input"} 
                    placeholder={"Insert block hash"}
                    type="text"
                />
                <div>
                    <label for="full">{"full"}</label>
                    <input type="checkbox" id={"full"} onchange={on_full_data_checkbox} checked={*full} />
                </div>
                if let Some(error) = (*error_msg).clone() {
                    <hr />
                    <small>{error}</small>
                }
                <button 
                    onclick={get_block} 
                    class="button"
                    disabled={(*hash).is_none()}
                >{"Get block"}</button>                
            }
        </div>
    }

}