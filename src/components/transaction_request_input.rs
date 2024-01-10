use yew::prelude::*;
use alloy_primitives::{Address, U128, U256, Bytes, U8, U64};
use alloy_rpc_types::{
    TransactionRequest, 
    AccessList, 
    CallInput,
    json_u256
};
use std::str::FromStr;
use crate::contexts::ethereum::UseEthereum;
use web_sys::HtmlInputElement;
use crate::components::{
    address_input::AddressInput,
};
/* 
TODO:
- error on Address encoding (no checksum?) 
    - if a do to_checksum() ends up in the same result? how does TransactionRequest encodes?
- handle error cases

*/

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_request: Callback<TransactionRequest>,
}

#[function_component(TransactionRequestInput)]
pub fn transaction_request_input(props: &Props) -> Html {
    //let from = use_state(|| None as Option<Address>);
    let to = use_state(|| None as Option<Address>);
    //let gas_price = use_state(|| None as Option<U128>);
    //let max_fee_per_gas = use_state(|| None as Option<U128>);
    //let max_priority_fee_per_gas = use_state(|| None as Option<U128>);
    //let gas = use_state(|| None as Option<U256>);
    let value = use_state(|| None as Option<U256>);
    let data = use_state(|| None as Option<Bytes>);
    //let nonce = use_state(|| None as Option<U64>);
    //let access_list = use_state(|| None as Option<AccessList>);
    //let transaction_type = use_state(|| None as Option<U8>);
    let error_msg = use_state(|| None as Option<String>);
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    //let chain = ethereum.chain();
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
        let f = ethereum.account().clone();
        let t = to.clone();
        let d = data.clone();
        let v = value.clone();
        let p = props.on_request.clone();
        Callback::from(move |_e: MouseEvent| {
            p.emit(
                TransactionRequest {
                    from: Some(f),
                    to: (*t).clone(),
                    gas_price: None,
                    max_fee_per_gas: None,
                    max_priority_fee_per_gas: None,
                    gas: None,
                    value: (*v).clone(),
                    data: (*d).clone(),
                    nonce: None,
                    access_list: None,
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
    let set_value = {
        let val = value.clone();
        let err = error_msg.clone();
        Callback::from(move |e: Event| {
            let h: HtmlInputElement = e.target_unchecked_into();
            let v = h.value();
            if let Ok(vv) = U256::from_str_radix(&v, 10) {
                val.set(Some(vv));
            } else {
                err.set(Some(String::from("Error on input")));
            }
        })
    };

    let set_data = {
        let t = data.clone();
        Callback::from(move |e: Event| {
            let h: HtmlInputElement = e.target_unchecked_into();
            let b = h.value();
            t.set(Some(Bytes::from_str(&b).expect("Error parsing bytes")));
        })
    };

    html!{
        <div class={"filter"}>
            if ethereum.is_connected() {
                <small>{"To"}</small>
                <AddressInput 
                    on_add={on_to}
                    show_me={true}
                    placeholder={String::from("Insert address to sent tx")}
                />
                <input onchange={set_value} placeholder={"value"} type="number" />
                <input onchange={set_data} placeholder={"data to call (byte string)"} class={"address_input"} type="text" /> 
                if (*error_msg).as_ref().is_some() {
                    <hr />
                    <p>{(*error_msg).clone()}</p>
                }
             
                <hr />
                <button onclick={create_tx_req} class={"button"}>{"Create Tx request"}</button>
            }
        </div>
    }
}