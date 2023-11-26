
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use alloy_chains::Chain;
//use serde::Serialize;
//use wasm_bindgen::JsValue;
//use crate::helpers::format_gas;

#[wasm_bindgen(module = "/src/jscripts/metamask.js")]
extern "C" {
    #[wasm_bindgen(js_name="connect")]
    #[wasm_bindgen(catch)]
    pub async fn connect() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name="getChainId")]
    #[wasm_bindgen(catch)]
    pub async fn getChainId() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_name="getClientVersion")]
    #[wasm_bindgen(catch)]
    pub async fn getClientVersion() -> Result<JsValue, JsValue>;
/* 

    #[wasm_bindgen(js_name="getChainId")]
    #[wasm_bindgen(catch)]
    pub async fn getChainId() -> Result<JsValue, JsValue>;
*/
}
pub enum WalletMsg {
    ConnectMetamask,
    GetChainId,
    SetError(String),
    SetAccount(String),
    SetChainId(Chain),
    GetClientVersion,
    SetClientVersion(String),
} 

pub struct Wallet {
    account: Option<String>,
    chain_id: Option<Chain>,
    version: Option<String>,
    pub errors: Option<String>,
}

impl Component for Wallet {
    type Message = WalletMsg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async move {
            WalletMsg::ConnectMetamask
        });
        Self {
            account: None,
            chain_id: None,
            version: None,
            errors: None,
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WalletMsg::ConnectMetamask => {
                ctx.link().send_future(async move {
                    match connect().await {
                        Ok(accs) => {
                            let account: Vec<String> = serde_wasm_bindgen::from_value(accs).unwrap();
                            WalletMsg::SetAccount(account.first().unwrap().to_string())
                        },
                        Err(_err) => {
                            WalletMsg::SetError("No metamask!".to_owned())
                        },
                    }
                });
                false
            }
            WalletMsg::SetError(err) => {
                self.errors = Some(err);
                true
            },
            WalletMsg::SetAccount(acc) => {
                self.account = Some(acc);
                ctx.link().send_message(WalletMsg::GetChainId);
                ctx.link().send_message(WalletMsg::GetClientVersion);
                true
            },
            WalletMsg::GetChainId => {
                ctx.link().send_future(async move {
                    match getChainId().await {
                        Ok(chain_id) => {
                            let chain_id_s: String = serde_wasm_bindgen::from_value(chain_id).unwrap();
                            let c = i64::from_str_radix(
                                &chain_id_s.trim_start_matches("0x"), 
                                16
                            ).unwrap();
                            let c = Chain::from_id(c.try_into().unwrap());
                            WalletMsg::SetChainId(c)
                        },
                        Err(_err) => WalletMsg::SetError("No chain".to_string())
                    }
                });
                false
            },
            WalletMsg::SetChainId(chain_id) => {
                self.chain_id = Some(chain_id);
                true
            },
            WalletMsg::GetClientVersion => {
                ctx.link().send_future(async move {
                    match getClientVersion().await {
                        Ok(v) => {
                            let version: String = serde_wasm_bindgen::from_value(v).unwrap();
                            WalletMsg::SetClientVersion(version)
                        },
                        Err(err) => {
                            let err_f: String = serde_wasm_bindgen::from_value(err).unwrap();
                            WalletMsg::SetError(err_f)
                        }
                    }
                });
                false
            },
            WalletMsg::SetClientVersion(v) => {
                self.version = Some(v);
                true
            }

        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        match (&self.account, &self.chain_id, &self.version) {
            (Some(acc), Some(chain_id), Some(version)) => {
                    html!(
                        <p>{format!("Connected as {} in {} with {}", acc, chain_id, version)}</p>
                    )
            },
            _ => {
                        html! (
                        <button
                            onclick={ctx.link().callback(|_| WalletMsg::ConnectMetamask)}
                        >{"connect"}</button>
                    )
            }
        }
    }
}
