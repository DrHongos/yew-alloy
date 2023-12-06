use alloy_dyn_abi::eip712::TypedData;
use alloy_sol_types::{Eip712Domain, SolStruct};
use alloy_sol_macro::sol;
use alloy_primitives::{U256, FixedBytes, Address, hex};
use alloy_dyn_abi::Resolver;
use crate::contexts::ethereum::UseEthereum;
use crate::helpers::log;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map};
use yew::{platform::spawn_local, prelude::*};
use std::borrow::Cow;
// eventually remove
use ethers::core::types::Signature;
use std::str::FromStr;

/* 
using 'ethers' only to recover signatures => find alloy's prospect into integrating this
*/

fn typed_data_for_document(name: String, chain_id_v: u64) -> TypedData {

    sol! {
        #[derive(Serialize)]
        struct DocumentSignature {
            string name;
            string content; 
        }
    };
    
    let domain = alloy_sol_types::eip712_domain!(
        name: name.clone(),
        version: "1",
        chain_id: chain_id_v,
    );
    let doc = DocumentSignature {
        name: (name.into()),
        content: "content of the doc".into(),
    };
    
    TypedData::from_struct(&doc, Some(domain))
}

#[function_component(SignatureButton)]
pub fn signature_button() -> Html {
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let chain_id = ethereum.chain_id();
    let onclick = {
        let ethereum = ethereum.clone();
        Callback::from(move |_: MouseEvent| {
            if ethereum.is_connected() {
                let data = typed_data_for_document("Rust Dapp".to_string(), chain_id);
                log(format!("TypedData {:#?}", data).as_str());
                
                let ethereum = ethereum.clone();
                spawn_local(async move {
                    let jason = json!(data).to_string();
                    //log(format!("Jason {:#?}", jason).as_str());
                    let signature_res = ethereum
                        .sign_typed_data(jason, &ethereum.account())
                        .await
                        .expect("Could not sign message");
                    log(format!("Signed message..{:#?}", &signature_res).as_str());

                    let signature_p = signature_res.strip_prefix("0x").unwrap();
                    let signature_p = hex::decode(signature_p).expect("Hex error");
                    let signature = Signature::try_from(signature_p.as_slice()).expect("Could not parse Signature");        
                    log(format!("Signature..{:#?}", signature).as_str());

                    // recover                    
                    let eip712_encoded_data = data.encode_data().expect("Could not encode eip712 data");
                    //log(format!("encoded eip712 data..{:#?}", eip712_encoded_data.to_string()).as_str());
                    let eip712_signing_hash = data.eip712_signing_hash().expect("Could not encode eip 712 signing hash");
                    log(format!("encoded signing hash..{:#?}", eip712_signing_hash).as_str());
                    // BUG: error on some part of the process as the recovered address isn't the signer
                    let rec = signature.recover(eip712_encoded_data.as_slice());                   
                    log(format!("Signing with {:?} recovered {:?}", ethereum.account(), rec).as_str());
                });
            } else {
                log("Are we disconnected?");
            }
        })
    };
    html! {
        <button {onclick} class={"button"} disabled={!ethereum.is_connected()}>{"Test signature"}</button>
    }
}
