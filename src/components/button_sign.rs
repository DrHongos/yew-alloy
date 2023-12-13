use alloy_dyn_abi::eip712::TypedData;
use alloy_sol_macro::sol;
use crate::contexts::ethereum::UseEthereum;
use crate::helpers::log;
use serde::Serialize;
use serde_json::json;
use yew::{platform::spawn_local, prelude::*};
use std::str::FromStr;
use alloy_signer::Signature;
use alloy_sol_types::Eip712Domain;

fn typed_data_for_document(name: String, chain_id_v: u64, content: String) -> TypedData {

    sol! {
        #[derive(Serialize)]
        struct DocumentSignature {
            string name;
            string content; 
        }
    };

    let doc = DocumentSignature {
        name: name.clone().into(),
        content: content,
    };
    let domain_obj: Eip712Domain = alloy_sol_types::eip712_domain!(
        name: name,
        version: "1",
        chain_id: chain_id_v,
    );

    let mut t = TypedData::from_struct(&doc, Some(domain_obj));
    // populates resolver with domain types
    let _ = t.resolver.ingest_string(t.domain().encode_type());
    t
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
                let data = typed_data_for_document("Rust Dapp".to_string(), chain_id, "Some content to sign".to_string());
                //log(format!("data: {:#?}",data).as_str());                
                let ethereum = ethereum.clone();
                
                spawn_local(async move {
                    
                    // handle signature rejected
                    let signature_res = ethereum
                        .sign_typed_data(json!(data).to_string(), &ethereum.account())
                        .await;

                    if let Ok(signed) = signature_res {
                        let signature = Signature::from_str(&signed).expect("Could not parse Signature");        
    
                        let msg_signing_hash = data.eip712_signing_hash().expect("No signing hash");
                        let rec = signature
                            .recover_address_from_prehash(
                                &msg_signing_hash
                            ).expect("Could not recover address from msg");
                        log(format!("Signing {:#?} with {:?} recovered {:?}", &data, ethereum.account(), rec).as_str());
                    } else {
                        log("Signature rejected");
                    }
                });

            } else {
                log("Are we disconnected?");
            }
        })
    };
    html! {
        <>
        <button {onclick} class={"button"} disabled={!ethereum.is_connected()}>{"Test signature"}</button>
        </>
    }
}
