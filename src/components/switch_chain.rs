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
    - error.. figure it out!    -> metamask request an object as argument
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
                let tdata = serde_json::json!(obj).to_string();
                let params: Cow<'static, String> = Cow::Owned(tdata);
                let client = client.clone();
                spawn_local(async move {
                    //if let Ok(client) = client {
                        //log(format!("{:#?}", &params).as_str());
                        let req: RpcCall<_, Cow<'static, String>, ()> = client.inner().prepare("wallet_switchEthereumChain", params);
                        match req.await {
                            Ok(()) => log(format!("Switched").as_str()),
                            Err(e) => log(format!("Error! {:#?}",e).as_str())
                        }     
                    //} else { log("not connected") }
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
//    let ontest = Callback::from(move |_: MouseEvent| {log(format!("{:#?}", enum_vector).as_str())});

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
                //<p>{chain.to_string()}</p>
                <button onclick={switch_chain} class="select-chain-button">{"Switch (NWY)"}</button>
                /* if chain != current_chain {
                } */
            }
        </div>
    }

}