use yew::prelude::*;
use alloy_primitives::{Address, U128, U256, Bytes, U8, U64};
use alloy_rpc_types::{CallRequest, AccessList, CallInput};
use std::str::FromStr;
//use crate::helpers::log;
use crate::contexts::ethereum::UseEthereum;
use web_sys::HtmlInputElement;
use crate::components::{
    address_input::AddressInput,
    blockid_input::BlockIdInput,
};
/* 
TODO:
    - create parameter encoder (enters method signature and allows to enter arguments, returns Bytes (for txrequest::data))
- handle error cases

*/

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_tx: Callback<CallRequest>,
}

#[function_component(TransactionRequestCreator)]
pub fn transaction_request_creator(props: &Props) -> Html {
    let from = use_state(|| None as Option<Address>);
    let to = use_state(|| None as Option<Address>);
    //let gas_price = use_state(|| None as Option<U128>);
    //let max_fee_per_gas = use_state(|| None as Option<U128>);
    //let max_priority_fee_per_gas = use_state(|| None as Option<U128>);
    //let gas = use_state(|| None as Option<U256>);
    //let value = use_state(|| None as Option<U256>);
    let data = use_state(|| None as Option<CallInput>);
    //let nonce = use_state(|| None as Option<U64>);
    //let access_list = use_state(|| None as Option<AccessList>);
    //let transaction_type = use_state(|| None as Option<U8>);

    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
/* 
    let set_block_hash = {
        let bh = block_hash.clone();
        Callback::from(move |e: Event| {
            let v: HtmlInputElement = e.target_unchecked_into();
            let p = FixedBytes::from_str(&v.value());
            if let Ok(p) = p {
                bh.set(B256::from(p))
            } else {
                bh.set(B256::from(FixedBytes::ZERO))
            }
        })
    };
 */
    let create_tx_req = {
        let f = from.clone();
        let t = to.clone();
        let d = data.clone();
        let p = props.on_tx.clone();
        Callback::from(move |_e: MouseEvent| {
            p.emit(
                CallRequest {
                    from: (*f).clone(),
                    to: (*t).clone(),
                    gas_price: None,
                    max_fee_per_gas: None,
                    max_priority_fee_per_gas: None,
                    gas: None,
                    value: None,
                    input: (*d).clone().unwrap(),
                    nonce: None,
                    chain_id: None,
                    access_list: None,
                    max_fee_per_blob_gas: None,
                    blob_versioned_hashes: None,
                    transaction_type: None,
                }
            )
        })
    };
    let on_to: Callback<Address> = {
        let t = to.clone();
        Callback::from(move |a| {
            t.set(Some(a));
        })
    };

    let set_data = {
        let t = data.clone();
        Callback::from(move |e: Event| {
            let h: HtmlInputElement = e.target_unchecked_into();
            let b = h.value();
            t.set(Some(CallInput::new(Bytes::from_str(&b).expect("Error parsing bytes"))));
        })
    };

    html!{
        <div class={"filter"}>
            if ethereum.is_connected() {
                <small>{"To"}</small>
                <AddressInput 
                    on_add={on_to}
                    show_me={true}
                    placeholder={String::from("Insert address to call to")}
                />
                <input onchange={set_data} placeholder={"data to call (byte string)"} class={"address_input"} type="text" /> 
                <hr />
                <button onclick={create_tx_req} class={"button"}>{"Create Tx request"}</button>
            }
        </div>
    }
}