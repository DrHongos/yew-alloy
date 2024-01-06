use yew::prelude::*;
use alloy_primitives::Address;
use alloy_rpc_client::RpcCall;
use std::{borrow::Cow, ops::Deref};
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use crate::contexts::ethereum::UseEthereum;
use crate::components::{
    get_code::GetCode,
    sign_typed_data::SignTypedData,
    switch_chain::SwitchChain,
    get_block_by_number::GetBlockByNumber,
    get_logs::GetLogs,
    get_balance::GetBalance,
};

#[function_component(ProviderTest)]
pub fn provider_tester() -> Html {
    
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let provider = ethereum.provider.clone();
    let main_account = ethereum.account();
    let logger = {
        let client = ethereum.clone();
        Callback::from(move |_: MouseEvent| {
            let client = client.clone();
            log(format!("{:#?}", client).as_str());
        })
    };

    let block_number = {
        let client = provider.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let client = client.clone();
                spawn_local(async move {
                    match client.get_block_number().await {
                        Ok(bn) => log(format!("Last block number: {}", bn).as_str()),
                        Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                    }
                })
            }
        })
    };
    let request_accounts = {
        let client = provider.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
            let client = client.clone();
            spawn_local(async move {
                match client.get_accounts().await {
                    Ok(bn) => log(format!("Accounts available: {:?}", bn).as_str()),
                    Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                }
            })
        } else { log("disconnected") }
    })};
        
    let client_version = {
        let client = provider.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let client = client.clone();
                let empty_params: Cow<'static, ()> = Cow::Owned(());
                spawn_local(async move {
                let req: RpcCall<_, Cow<'static, ()>, String> = client.inner().prepare("web3_clientVersion", empty_params.clone());
                match req.await {
                    Ok(d) => log(format!("{:#?}", d).as_str()),
                    _ => log("Error")
                } 
            })
        } else { log("disconnected") }
    })};

    let chain_id = {
        let client = provider.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let client = client.clone();
                spawn_local(async move {
                    match client.get_chain_id().await {
                        Ok(c) => log(format!("Chain id: {}", c).as_str()),
                        Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                    }
                })
            } else { log("disconnected") }
        })};

    let get_tx_count = {
        let client = provider.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let account: Address = main_account.into();
                let client = client.clone();
                spawn_local(async move {
                    match client.get_transaction_count(account).await {
                        Ok(b) => log(format!("Transaction count for {} is {}", account, b).as_str()),
                        Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                    }
                })
            }
        })
    };

    let get_gas_price = {
        let client = provider.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let client = client.clone();
                spawn_local(async move {
                    match client.get_gas_price().await {
                        Ok(b) => log(format!("Gas price is {}", b).as_str()),
                        Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                    }
                })
            }
        })
    };

    let syncing = {
        let client = provider.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let client = client.clone();
                spawn_local(async move {
                    match client.syncing().await {
                        Ok(b) => log(format!("Syncing data is {:#?}", b).as_str()),
                        Err(rv) => log(format!("Error: {:#?}", rv).as_str())
                    }
                })
            }
        })
    };

    html!{
        <div class={"rpc_tester"}>
            if ethereum.is_connected() {
                <button onclick={logger} class="button">{"log provider"}</button>
                <button onclick={block_number} class="button">{"blockNumber"}</button>
                <button onclick={request_accounts} class="button">{"requestAccounts"}</button>
                <button onclick={client_version} class="button">{"clientVersion"}</button>
                <button onclick={chain_id} class="button">{"chainId"}</button>
                <button onclick={get_tx_count} class="button">{"TxCount"}</button>
                <button onclick={get_gas_price} class="button">{"GasPrice"}</button>
                <button onclick={syncing} class="button">{"Syncing"}</button>
                <div class="shorts">
                    <GetBalance />
                    <GetCode />
                    <SwitchChain />
                </div>
                <div class="shorts">
                    <GetBlockByNumber />
                    <SignTypedData />
                </div>
                <GetLogs />
            }
        </div>
    }

}