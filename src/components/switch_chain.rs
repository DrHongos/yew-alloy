use yew::prelude::*;
use alloy_rpc_client::RpcCall;
use std::{borrow::Cow, ops::Deref};
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use crate::contexts::ethereum::UseEthereum;
use alloy_chains::{Chain, NamedChain};
use web_sys::HtmlInputElement;
use strum::IntoEnumIterator;
use serde::Serialize;
/* 
TODO:
    - handle chain not known ("you should add this chain first")
    - initial state (use_effect)
    - disable current chain
    - handle error cases
*/

#[derive(Serialize, Clone, Debug)]
pub struct SwitchChainObj {
    #[serde(rename = "chainId")]
    chain_id: String,
}

#[function_component(SwitchChain)]
pub fn switch_chain() -> Html {
    let chain = use_state(|| "0x1".to_string());            // use_effect to set it initially?    
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let client = ethereum.provider.clone();
    let switch_chain = {
        let client = client.clone();
        let chain = chain.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(client) = client.deref() {
                let obj = SwitchChainObj {
                    chain_id: (*chain).clone() 
                };
                let params: Cow<'static, SwitchChainObj> = Cow::Owned(obj);
                let client = client.clone();
                spawn_local(async move {
                    let req: RpcCall<_, Vec<Cow<'static, SwitchChainObj>>, ()> = client.inner().prepare("wallet_switchEthereumChain", vec![params]);
                    match req.await {
                        Ok(()) => log(format!("Switched").as_str()),
                        Err(e) => log(format!("Error! {:#?}",e).as_str())
                    }     
                })
            }
        })
    };
    
    let on_change_chain = {
        let c = chain.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let ns = input.value();
            c.set(format!("0x{}", ns));
        })
    };
    
    let enum_iter = NamedChain::iter();
    let list: Vec<NamedChain> = enum_iter.collect();

    html!{
        <div class={"getCode"}>
            if ethereum.is_connected() {
                <select class={"select-chain"} onchange={on_change_chain}>
                    {
                        list.into_iter().map(|c| {
                            html!{
                                <option class={"option-chain"} value={Chain::from(c).id().to_string()}>{c.as_str()}</option>
                            }
                        }).collect::<Html>()
                    }
                </select>
                <button onclick={switch_chain} class="select-chain-button">{"Switch chain"}</button>
            }
        </div>
    }

}