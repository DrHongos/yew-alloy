use yew::prelude::*;
use alloy_primitives::Address;
use alloy_rpc_types::BlockNumberOrTag;
use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;
use foundry_block_explorers::contract::ContractMetadata;
use crate::helpers::log;
use crate::contexts::ethereum::UseEthereum;
use crate::components::address_input::AddressInput;

/* 
TODO:
    - create a contract abi representation to display 
    - add tools to create listeners/send transactions from it
    - handle error cases
*/
#[function_component(GetCode)]
pub fn get_code() -> Html {    
    let address = use_state(|| None as Option<Address>);
    let contract_metadata = use_state(|| None as Option<ContractMetadata>);
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let provider = ethereum.provider.clone();
    let etherscan_client = ethereum.etherscan_client.clone();

    let get_code_address = {
        let addr = (*address).clone();
        let chain = ethereum.chain();
        let contract_metadata = contract_metadata.clone();
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
                        spawn_local(async move {
                            let metadata = client.contract_source_code(addr).await.expect("Cannot get code");
                            log(format!("{:#?}", metadata).as_str());
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

    let contracts_list = (*contract_metadata)
        .clone()
        .unwrap_or(foundry_block_explorers::contract::ContractMetadata { items: Vec::new() })
        .items
        .into_iter()
        .map(|a| 
            html!(
                <div>
                    <p>{a.contract_name}</p>
                </div>
            )
        )
        .collect::<Html>();
    
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
//                    <p>{format!("{} contracts loaded", contracts.items.len()).as_str()}</p>
                }
            }
        </div>
    }

}