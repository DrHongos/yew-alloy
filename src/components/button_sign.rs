use alloy_dyn_abi::eip712::TypedData;
use alloy_sol_types::Eip712Domain;
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
take from the ecrecover method
https://github.com/ethereumjs/ethereumjs-monorepo/blob/master/packages/util/src/signature.ts
https://github.com/MetaMask/eth-sig-util/blob/main/src/utils.ts

using 'ethers' only to recover signatures => find alloy's prospect into integrating this

*/

const DOCUMENT_SIGNATURE_NAME: &str = "DocumentSignature";
const VERIFIER_NAME: &str = "Test App";

fn typed_data_for_document(name: &str, chain_id_v: u64) -> TypedData {

     sol! {
        struct EIP712Domain {
            string name;
            string version;
            uint256 chainId;
            address verifyingContract;
            bytes32 salt;
        }
        struct DocumentSignature {
            string name;
            string content; 
        }
    };
    // create resolver
    let mut graph = Resolver::default();
    graph.ingest_sol_struct::<EIP712Domain>();
    graph.ingest_sol_struct::<DocumentSignature>();

    TypedData {
        domain: Eip712Domain {
            name: Some(Cow::Borrowed(VERIFIER_NAME)),
            version: Some(Cow::Borrowed("1")),
            chain_id: Some(U256::from(chain_id_v)),
            verifying_contract: Some(Address::ZERO),
            salt: Some(FixedBytes::ZERO),
        },
        resolver: graph,
        primary_type: DOCUMENT_SIGNATURE_NAME.to_string(),
        message: DocumentDescription::new(name).into_value(),
    }

/* 
    // example test 
        let s: FixedBytes<32> = FixedBytes::ZERO;
        let json = json!({
            "types": {
                "EIP712Domain": [
                    {
                        "name": "name",
                        "type": "string"
                    },
                    {
                        "name": "version",
                        "type": "string"
                    },
                    {
                        "name": "chainId",
                        "type": "uint256"
                    },
                    {
                        "name": "verifyingContract",
                        "type": "address"
                    },
                    {
                        "name": "salt",
                        "type": "bytes32"
                    }
                ]
            },
            "primaryType": "EIP712Domain",
            "domain": {
                "name": "example.metamask.io",
                "version": "1",
                "chainId": 1,
                "verifyingContract": "0x0000000000000000000000000000000000000000",
                "salt": s
            },
            "message": {}
        });

        let typed_data: TypedData = serde_json::from_value(json).unwrap();
        
        // test? FAILED!
        let hash = typed_data.eip712_signing_hash().unwrap();
        log(format!(
            "signature hash is {:#?} and should be 122d1c8ef94b76dad44dcb03fa772361e20855c63311a15d5afe02d1b38f6077",
            hex::encode(hash)
        ).as_str());
        //
        typed_data
        // end of example
 */
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDescription {
    pub name: String,
    pub content: String,
}

impl DocumentDescription {

    pub fn into_value(&self) -> serde_json::Value {
        let mut m = Map::new();
        m.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        m.insert("content".to_string(), serde_json::Value::String(self.content.clone()));
        serde_json::Value::Object(m)
    }

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            content: format!("By signing this message you comply with {}. This request will not trigger a blockchain transaction or cost any gas fees.", name),
        }
    }
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
                let data = typed_data_for_document("Content of this Document", chain_id);
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
                    let signature = Signature::from_str(&signature_res).expect("Could not parse signature");
                    log(format!("Signature..{:#?}", signature).as_str());

                    // recover
                    let eip712_encoded_data = data.encode_data().expect("Could not encode eip712 data");
                    log(format!("encoded eip712 data..{:#?}", eip712_encoded_data).as_str());
                    
                    // BUG: error on some part of the process as the recovered address isn't the signer
                    let rec = signature.recover(eip712_encoded_data);
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
