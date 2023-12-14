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
use web_sys::HtmlInputElement;

fn typed_data_for_document(name: &str, chain_id_v: u64, content: &str) -> TypedData {

    sol! {
        #[derive(Serialize)]
        struct DocumentSignature {
            string name;
            string content; 
        }
    };

    let doc = DocumentSignature {
        name: name.to_string(),
        content: content.to_string(),
    };
    let domain_obj: Eip712Domain = alloy_sol_types::eip712_domain!(
        name: "AlloYew",
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
    let msg = use_state(|| "Some content to sign".to_string());
    let name = use_state(|| "Your name".to_string());

    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let chain_id = ethereum.chain_id();

    let onclick = {
        let ethereum = ethereum.clone();
        let message = (*msg).clone();
        let name = (*name).clone();
        Callback::from(move |_: MouseEvent| {
            if ethereum.is_connected() {
                let data = typed_data_for_document(&name, chain_id, &message);
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

    let on_change_message = {
        let message = msg.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            message.set(input.value());
        })
    };
    let on_change_name = {
        let naming = name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            naming.set(input.value());
        })
    };

    html! {
        <div class={"signer"}>
        
        <label>{"Name"}</label>
        <input onchange={on_change_name} class={"name_input"} disabled={!ethereum.is_connected()} type="text" value={(*name).clone()} />
        <br />
        <label>{"Content"}</label>
        <input onchange={on_change_message} class={"msg_input"} disabled={!ethereum.is_connected()} type="text" value={(*msg).clone()} />
        <hr />
        <button {onclick} class={"button"} disabled={!ethereum.is_connected()}>{"Sign it"}</button>
        </div>
    }
}
