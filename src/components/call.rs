use yew::prelude::*;
use crate::contexts::ethereum::UseEthereum;
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use std::ops::Deref;
use crate::components::{
    block_selector::BlockSelector,
    blockid_input::BlockIdInput,
    transaction_request_creator::TransactionRequestCreator,
};
use std::ops::BitXorAssign;
use alloy_rpc_types::{BlockId, CallRequest};
/* 
TODO:
    - component to create and make call() queries
    - create and test fake transactionrequest
        ie: DAI totalSupply()
        to: 0x6B175474E89094C44Da98b954EedeAC495271d0F
        data: 0x18160ddd        (to get more use: cast calldata "method(args)" args)
*/

#[function_component(Call)]
pub fn call() -> Html {
    let open = use_state(|| false);
    let block = use_state(|| None as Option<BlockId>);
    let tx = use_state(|| None as Option<CallRequest>);
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let client = ethereum.provider.clone();

    let on_block_entry: Callback<BlockId> = {
        let b = block.clone();
        Callback::from(move |inp: BlockId| {
            log(format!("Received blockId {:#?}", inp).as_str());
            b.set(Some(inp));
        })
    };
    let on_tx: Callback<CallRequest> = {
        let t = tx.clone();
        Callback::from(move |txr| {
            log(format!("Received CallRequest {:#?}", txr).as_str());
            t.set(Some(txr));
        })
    };
    let call = {
        let tx = (*tx).clone();
        let b = (*block).clone();
        let client = client.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let client = client.clone();
                let b = b.clone();
                let tx = tx.clone();
                spawn_local(async move {
                    match client.call(tx.expect("No txr"), b).await {
                        Ok(bn) => log(format!("Call result: {}", bn).as_str()),
                        Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                    }
                })
            }
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
                    <TransactionRequestCreator 
                        on_tx={on_tx}
                    />
                    <BlockIdInput 
                        on_block_entry={on_block_entry}
                    />
                    if (*tx).is_some() && (*block).is_some() {
                        <button
                            onclick={call}
                            class={"button"}
                        >{"Call"}</button>
                    }
                }
            }
        </div>
    }

}