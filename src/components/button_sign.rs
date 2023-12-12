use alloy_dyn_abi::eip712::TypedData;
use alloy_sol_macro::sol;
use crate::contexts::ethereum::UseEthereum;
use crate::helpers::log;
use serde::Serialize;
use serde_json::json;
use yew::{platform::spawn_local, prelude::*};
use std::str::FromStr;
use alloy_signer::Signature;

fn typed_data_for_document(name: String, chain_id_v: u64) -> TypedData {
    sol! {
        #[derive(Serialize)]
        struct DocumentSignature {
            string name;
            string content; 
        }
    };
/*     
    let domain = alloy_sol_types::eip712_domain!(
        name: name.clone(),
        version: "1",
        chain_id: chain_id_v,
    );
*/
    let doc = DocumentSignature {
        name: name.into(),
        content: "content of the doc".into(),
    };
    
    // only can recover like this. if Domain is in, won't work
    let t = TypedData::from_struct(&doc, None /* Some(domain) */);  
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
                let data = typed_data_for_document("Rust Dapp".to_string(), chain_id);
                log(format!("{:#?}",data).as_str());                
                let ethereum = ethereum.clone();
                spawn_local(async move {
                    let signature_res = ethereum
                        .sign_typed_data(json!(data).to_string(), &ethereum.account())
                        .await
                        .expect("Could not sign message");

                    let signature = Signature::from_str(&signature_res).expect("Could not parse Signature");        

                    let rec = signature
                        .recover_address_from_prehash(
                            &data.eip712_signing_hash().expect("Couldnt encode data")
                        ).expect("Could not recover address from msg");
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
