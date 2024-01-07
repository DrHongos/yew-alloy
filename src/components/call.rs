use yew::prelude::*;
use crate::contexts::ethereum::UseEthereum;
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use std::ops::Deref;
use crate::components::{
    block_selector::BlockSelector,
    blockid_input::BlockIdInput,
};
use std::ops::BitXorAssign;
use alloy_rpc_types::BlockId;
/* 
TODO:
    - component to create and make call() queries

*/

#[function_component(Call)]
pub fn call() -> Html {
    let open = use_state(|| false);
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let client = ethereum.provider.clone();
    let block_selected = use_state(|| None as Option<BlockId>);

    //    let logs = use_state(|| Vec::<Log>::new());
    /* 
    let on_filter: Callback<Filter> = {
        let f = filter.clone();
        let client = client.clone();
        Callback::from(move |inp: Filter| {
            f.set(inp.clone());
            let client = client.clone();
            let inp = inp.clone();
            spawn_local(async move {
                if let Some(client) = client.deref() {
                    match client.get_logs(inp).await {
                        Ok(d) => log(format!("latest logs are {:#?}", d).as_str()),
                        _ => log("Error!")
                    }     
                } else { log("not connected") }
            })
        })
    };
    */
    let on_block_entry: Callback<BlockId> = {
        let b = block_selected.clone();
        Callback::from(move |inp: BlockId| {
            log(format!("Received blockId {:#?}", inp).as_str());
            b.set(Some(inp));
        })
    };

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
        true => "Cancel",
        false => "Call",
    }; 
    html!{
        <div class={"getCode"}>
            if ethereum.is_connected() {
                <button onclick={toggle_comp} class={"button"}>
                    {button_label}
                </button>
                if (*open).clone() {
                    <p>{"Create component"}</p>
                    <p>{"TransactionRequest"}</p>
                    <BlockIdInput 
                        on_block_entry={on_block_entry}
                    />
                }
                // collect/display logs?
            }
        </div>
    }

}

// alloy_rpc_types::request::TransactionRequest
/* 
pub struct TransactionRequest {
    /// from address
    pub from: Option<Address>,
    /// to address
    pub to: Option<Address>,
    /// legacy, gas Price
    #[serde(default)]
    pub gas_price: Option<U128>,
    /// max base fee per gas sender is willing to pay
    #[serde(default)]
    pub max_fee_per_gas: Option<U128>,
    /// miner tip
    #[serde(default)]
    pub max_priority_fee_per_gas: Option<U128>,
    /// gas
    pub gas: Option<U256>,
    /// value of th tx in wei
    pub value: Option<U256>,
    /// Any additional data sent
    #[serde(alias = "input")]
    pub data: Option<Bytes>,
    /// Transaction nonce
    pub nonce: Option<U64>,
    /// warm storage access pre-payment
    #[serde(default)]
    pub access_list: Option<AccessList>,
    /// EIP-2718 type
    #[serde(rename = "type")]
    pub transaction_type: Option<U8>,
} 
*/