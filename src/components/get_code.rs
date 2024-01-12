use yew::prelude::*;
use alloy_primitives::{Address, Bytes};
use alloy_rpc_types::{BlockNumberOrTag, CallRequest, CallInput};
use std::{ops::Deref, str::FromStr};
use wasm_bindgen_futures::spawn_local;
use foundry_block_explorers::contract::ContractMetadata;
use crate::helpers::log;
use crate::contexts::ethereum::UseEthereum;
use crate::components::address_input::AddressInput;
use alloy_json_abi::{JsonAbi, Function, StateMutability};
use web_sys::HtmlButtonElement;
/* 
TODO:
    -  display contract abi 
    https://docs.rs/alloy-json-abi/0.6.0/alloy_json_abi/struct.JsonAbi.html

    create Function filler  -> Send/Call
        https://docs.rs/alloy-json-abi/0.6.0/alloy_json_abi/struct.Function.html

    create Filter based on contract -> Address + Events + others    

    - handle error cases
*/
#[function_component(GetCode)]
pub fn get_code() -> Html {    
    let address = use_state(|| None as Option<Address>);
    let contract_metadata = use_state(|| None as Option<ContractMetadata>);
    let contract_abi = use_state(|| None as Option<JsonAbi>);
    let contract_functions = use_state(|| None as Option<Vec<Vec<Function>>>);
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let provider = ethereum.provider.clone();
    let etherscan_client = ethereum.etherscan_client.clone();

    let get_code_address = {
        let addr = (*address).clone();
        let chain = ethereum.chain();
        let contract_metadata = contract_metadata.clone();
        let contract_abi = contract_abi.clone();
        let contract_functions = contract_functions.clone();
        let provider = provider.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(provider) = provider.deref() {
                if let Some(addr) = addr {
                    let provider = provider.clone();
                    spawn_local(async move {
                        match provider.get_code_at(addr.clone(), BlockNumberOrTag::Latest).await {
                            Ok(d) => log(format!("Code in {} address: {} is {:#?}", chain.unwrap(), &addr, d).as_str()),
                            Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                        };
                    });
                    if let Some(client) = etherscan_client.deref() {
                        let client = client.clone();
                        let cs = contract_metadata.clone();
                        let cabi = contract_abi.clone();
                        let cfns = contract_functions.clone();
                        spawn_local(async move {
                            // somethin wrong with Address definition below
                            let metadata = client.clone().contract_source_code(addr.clone().to_string().parse().unwrap()).await.expect("Cannot get code");
                            let f = metadata.items.first().unwrap();
                            let abi = f.abi().expect("Cannot get ABI");
                            //log(format!("{:#?}", abi).as_str());
                            let functions_list = abi
                                .clone()
                                .functions
                                .values()
                                .map(|f| f.clone())
                                .collect::<Vec<Vec<Function>>>();
                            //log(format!("Functions {:#?}", functions_list).as_str());
                            if functions_list.len() > 0 {
                                cfns.set(Some(functions_list));
                            }
                            cabi.set(Some(abi));
                            cs.set(Some(metadata));
                        });
                    }
                }
            } else { log("not connected") }
        })
    };

    let on_add_address: Callback<Address> = {
        let a = address.clone();
        Callback::from(move |addr| {
            a.set(Some(addr));
        })
    };

    // zip abi?
    let contracts_list = (*contract_metadata)
        .clone()
        .unwrap_or(foundry_block_explorers::contract::ContractMetadata { items: Vec::new() })
        .items
        .into_iter()
        .map(|a| 
            html!(
                <div>
                    <p>{a.contract_name}</p>                    
                    //<p>{format!("Methods: {}", serde_json::json!(a.abi).iter().filter(|e| e.type == "function").collect())}</p>
                    //<p>{format!("Events: {}", a.abi.iter().filter(|e| e.type == "event").collect())}</p>
                </div>
            )
        )
        .collect::<Html>();
        
    let call_function = {
        let provider = provider.clone();
        let address = (*address).clone();
        Callback::from(move |e: MouseEvent| {
            let t: HtmlButtonElement = e.target_unchecked_into();
            let fn_selector = t.value();
            let tx = CallRequest {
                to: address,
                input: CallInput::new(Bytes::from_str(&fn_selector).expect("Error parsing bytes")),
                from: None,
                gas_price: None,
                max_fee_per_gas: None,
                max_priority_fee_per_gas: None,
                gas: None,
                value: None,
                nonce: None,
                chain_id: None,
                access_list: None,
                max_fee_per_blob_gas: None,
                blob_versioned_hashes: None,
                transaction_type: None,
            };
            let provider = provider.clone();
            spawn_local(async move {
                log(format!("tx is {:#?}", tx).as_str());
                if let Some(provider) = (*provider).clone() {
                    let c = provider.call(tx, None);
                    match c.await {
                        Ok(bn) => log(format!("Call result: {}", bn).as_str()),
                        Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                    }
                }
            })
        })
    };

    let functions_list_obj = (*contract_functions)
        .clone()
        .unwrap_or(Vec::new())
        .into_iter()
        .map(|v|  {
            let v = v.first().unwrap();
            let selector = v.selector().to_string();
            html!(
                <div class={"contract_function"}>                    
                    <b>{&v.name}</b>
                    <small>{format!(" [{:?}]", v.state_mutability)}</small>
                    if v.inputs.len() == 0 && v.state_mutability != StateMutability::Payable {
                        <button onclick={call_function.clone()} value={selector} class={"button"}>{"Call"}</button>
                    }
                    //                    <hr />
                    // add inputs reflecting the Params
                </div>
            )}
        ).collect::<Html>();

    html!{
        <div class={"getCode"}>
            if ethereum.is_connected() {
                <AddressInput 
                    on_add={on_add_address}
                    show_me={false}
                    placeholder={String::from("Insert contract address")}
                />
                if (*address).clone().is_some() {
                    <button onclick={get_code_address} class="button">{"Get code"}</button>
                }
                if let Some(_contracts) = (*contract_metadata).clone() {
                    <hr />
                    {contracts_list}
                }
                if let Some(abi) = (*contract_abi).clone() {
                    <hr />
                    <p>{format!("Functions: {}", abi.functions.len())}</p>
                    <p>{format!("Events: {}", abi.events.len())}</p>
                }
                {functions_list_obj}
            }
        </div>
    }

}