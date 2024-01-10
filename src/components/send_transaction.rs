use yew::prelude::*;
use alloy_primitives::Address;
use alloy_rpc_types::{BlockId, TransactionRequest};
use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;
use std::ops::BitXorAssign;
use crate::helpers::log;
use crate::contexts::ethereum::UseEthereum;
use crate::components::{
    address_input::AddressInput,
    blockid_input::BlockIdInput,
    transaction_request_input::TransactionRequestInput,
};
/* 
TODO:
    - create TransactionRequest
    - sign and send a transaction
    - handle error cases

*/

#[function_component(SendTransaction)]
pub fn send_transaction() -> Html {
    let open = use_state(|| false); 
    let request = use_state(|| None as Option<TransactionRequest>);
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let provider = ethereum.provider.clone();

    let send_transaction = {
        let client = provider.clone();
        let request = request.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let client = client.clone();
                let request = request.clone();
                spawn_local(async move {
                    if let Some(req) = (*request).clone() {
                        crate::helpers::log(format!("TransactionRequest: {:#?}", req).as_str());
                        match client.send_transaction(req).await {
                            Ok(b) => log(format!("Tx hash: {}", b).as_str()),
                            Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                        }
                    }
                })
                } else { log("disconnected") }
        }
    )};


    let onset_request: Callback<TransactionRequest> = {
        let r = request.clone();
        Callback::from(move |txr| {
            r.set(Some(txr));
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
        false => "Send Transaction",
    };

    html!{
        <div class={"getCode"}>
            if ethereum.is_connected() {
                <button onclick={toggle_comp} class={"button"}>
                    {button_label}
                </button>
                if (*open).clone() {
                    <div>
                        <TransactionRequestInput 
                            on_request={onset_request}
                        />
                        if (*request).clone().is_some() {
                            <button 
                                onclick={send_transaction} 
                                class="button"
                                disabled={(*request).clone().is_none()}
                            >{"Sign and send"}</button>
                        }
                    </div>
                }
            }
        </div>
    }

}