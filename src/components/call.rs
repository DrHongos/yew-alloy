use yew::prelude::*;
use crate::contexts::ethereum::UseEthereum;
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use crate::components::{
    blockid_input::BlockIdInput,
    call_request_input::CallRequestInput, 
};
use std::{
    ops::{Deref, BitXorAssign},
    str::FromStr,
};
use alloy_rpc_types::{BlockId, CallRequest, CallInput};
use alloy_primitives::{address, Bytes};

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
            b.set(Some(inp));
        })
    };
    let on_tx: Callback<CallRequest> = {
        let t = tx.clone();
        Callback::from(move |txr| {
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
                    let tx = tx.expect("No txr");
                    log(format!("tx is {:#?}", tx).as_str());
                    let c = client.call(tx, b);
                    match c.await {
                        Ok(bn) => log(format!("Call result: {}", bn).as_str()),
                        Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                    }
                })
            }
        })
    };
    let test_dai_supply = {
        let b = (*block).clone();
        let client = client.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let client = client.clone();
                let b = b.clone();
                spawn_local(async move {
                    let tx = CallRequest {
                        from: None,
                        to: Some(address!("6B175474E89094C44Da98b954EedeAC495271d0F")), // DAI [mainnet]
                        gas_price: None,
                        max_fee_per_gas: None,
                        max_priority_fee_per_gas: None,
                        gas: None,
                        value: None,
                        input: CallInput::new(Bytes::from_str("0x18160ddd").expect("Error parsing bytes")), // totalSupply()
                        nonce: None,
                        access_list: None,
                        transaction_type: None,
                        chain_id: None,
                        max_fee_per_blob_gas: None,
                        blob_versioned_hashes: None,
                    };
                    //log(format!("tx is {:#?}", tx).as_str());
                    let c = client.call(tx, b);
                    match c.await {
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
                    <button onclick={test_dai_supply} class={"button"}>{"mainnet::DAI::totalSupply()"}</button>
                    <hr />
                    <CallRequestInput 
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
