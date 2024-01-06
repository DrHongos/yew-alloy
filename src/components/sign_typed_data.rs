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
use std::borrow::Cow;
use alloy_rpc_client::RpcCall;

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

    TypedData::from_struct(&doc, Some(domain_obj))
}

#[function_component(SignTypedData)]
pub fn sign_typed_data() -> Html {
    let msg = use_state(|| "Some content to sign".to_string());
    let name = use_state(|| "Your name".to_string());

    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let chain_id: u64 = ethereum.chain().unwrap_or_default().id();

    let onclick = {
        let ethereum = ethereum.clone();
        let message = (*msg).clone();
        let name = (*name).clone();
        Callback::from(move |_: MouseEvent| {
            let provider = ethereum.provider.as_ref().clone().expect("Disconnected");
            let account = ethereum.main_account();
            let data = typed_data_for_document(&name, chain_id, &message);
            //log(format!("data: {:#?}",data).as_str());
            let ethereum = ethereum.clone();
            let client = provider.clone();
            spawn_local(async move {
                let tdata = json!(data).to_string();
                let params1: Cow<'static, String> = Cow::Owned(account);
                let params2: Cow<'static, String> = Cow::Owned(tdata);

                let req: RpcCall<_, Vec<Cow<'static, String>>, String> = client.inner().prepare("eth_signTypedData_v4", vec![params1, params2]);

                if let Ok(signed) = req.await {
                    let signature = Signature::from_str(&signed).expect("Could not parse Signature");        

                    let msg_signing_hash = data.eip712_signing_hash().expect("No signing hash");
                    let rec = signature
                        .recover_address_from_prehash(
                            &msg_signing_hash
                        ).expect("Could not recover address from msg");
                    log(format!("Signature is: {}", &signed).as_str());
                    log(format!("{:#?} signed with {:?} recovered {:?}", &data, ethereum.account(), rec).as_str());
                } else {
                    log("Signature rejected");
                }
            });
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
            <input onchange={on_change_name} class={"name_input"} disabled={!ethereum.is_connected()} type="text" value={(*name).clone()} />
            <textarea onchange={on_change_message} class={"msg_input"} disabled={!ethereum.is_connected()} type="text" value={(*msg).clone()} />
            <button {onclick} class={"button-sign"} disabled={!ethereum.is_connected()}>{"Sign typed data"}</button>
        </div>
    }
}
