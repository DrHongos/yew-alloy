use yew::prelude::*;
use crate::contexts::ethereum::UseEthereum;
use alloy_rpc_types::BlockNumberOrTag;
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use std::ops::{Deref, BitXorAssign};
use crate::components::block_selector::BlockSelector;
/* 
TODO:
    - handle error cases
    - display info
*/

#[function_component(GetBlockByNumber)]
pub fn get_block_by_number() -> Html {
    let complete = use_state(|| false);
    let block = use_state(|| None as Option<BlockNumberOrTag>);
    //let block = use_state(|| None::<Block>);
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let client = ethereum.provider.clone();
    
    let get_block = {
        let client = client.clone();
        if let Some(block) = (*block).clone() {
            let complete = complete.clone();
            Callback::from(move |_: MouseEvent| {
                let client = client.clone();
                let block = block.clone();
                let complete = (*complete).clone();
                spawn_local(async move {
                    if let Some(client) = client.deref() {
                        match client.get_block_by_number(block.clone(), complete).await {
                            Ok(d) => log(format!("block {} is {:#?}", block, d).as_str()),
                            _ => log("Error!")
                        }     
                    } else { log("not connected") }
                })
                
            })
        } else {
            Callback::from(|_| {log("Select a block")})
        }
    };

    let on_block_entry: Callback<(bool, BlockNumberOrTag)> = {
        let b = block.clone();
        Callback::from(move |inp| {
            let (_from, nblock) = inp;
            b.set(Some(nblock));
        })
    };

    let on_full_data_checkbox = {
        let c = complete.clone();
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
                <BlockSelector 
                    from={true}
                    on_block_entry={on_block_entry.clone()}
                />
                <label for="complete">{"complete data"}</label>
                <input type="checkbox" id={"complete"} onchange={on_full_data_checkbox} checked={*complete} />
                <button 
                    onclick={get_block} 
                    class="button"
                    disabled={(*block).is_none()}
                >{"Get block"}</button>                
            }
        </div>
    }

}