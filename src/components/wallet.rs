use wasm_bindgen::prelude::*;
use yew::prelude::*;
use alloy_chains::Chain;
use crate::components::wallet::html::Scope;
use crate::eip1193::Provider;

/* 
TODO:
- implement listeners
- get balance

*/

#[wasm_bindgen]
extern "C" {
    // if i have a "Provider" like object, i can simply call "requests" (https://github.com/ZeroProphet/EIP1193_rs/blob/master/example/yewapp/)
    #[wasm_bindgen(js_namespace=["window","ethereum"], js_name="request")]
    pub fn request(m: &str);

    #[wasm_bindgen(js_namespace=["console"])]
    pub fn log(value: &str);
    /*   
        #[wasm_bindgen(js_namespace=["window","ethereum"]/* , js_name="on" */)]
        pub fn on(m: &str);
     */
}
pub enum WalletMsg {
    ConnectMetamask,
//    Disconnect,
    GetChainId,
    SetError(String),
    SetAccount(JsValue),
    SetChainId(Chain),
    GetClientVersion,
    SetClientVersion(String),
} 

pub struct Wallet {
    account: Option<String>,
    chain_id: Option<Chain>,
    version: Option<String>,
    pub errors: Option<String>,
    // eip1193 method
    provider: Provider,
    

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
            provider: Provider::new(),
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WalletMsg::ConnectMetamask => {
// method with eip1193 Provider object 
                let provider = self.provider.clone();
                let link = ctx.link().clone();
                provider.request(
                    "eth_requestAccounts".into(),
                    None,
                    Box::new(link),
                    Box::new(|a, b| {
                        let link = b.downcast_ref::<Scope<Wallet>>().unwrap();
                        match a {
                            Ok(r) => {
                                link.send_message(
                                    WalletMsg::SetAccount(r)
                                );
                            }
                            Err(_e) => {
                                link.send_message(WalletMsg::SetError("Error on callback".to_string()))
                            }
                        }
                    })
                );
                false
            }
            WalletMsg::SetError(err) => {
                self.errors = Some(err);
                true
            },
            /* 
            WalletMsg::Disconnect => {
                self.account = None;
                self.chain_id = None;
                let provider = self.provider.clone();
                provider.on(
                    "disconnect".into(), 
                    Box::new(|_| {})
                ).expect("No disconnect callback set");
                true
            }, 
            */
            WalletMsg::SetAccount(accs) => {
                let accounts: Vec<String> = serde_wasm_bindgen::from_value(accs).unwrap();
                if let Some(acc) = accounts.first() {
                    self.account = Some(acc.to_string());
                    ctx.link().send_message(WalletMsg::GetChainId);
                    ctx.link().send_message(WalletMsg::GetClientVersion);
                    let link = ctx.link().clone();
                    let provider = self.provider.clone();
                    
                    
                    provider.on(
                        "accountsChanged".into(), 
                        //"chainChanged".into(),
                        Box::new(
                            move |s| {
                            log(format!("{:#?}", &s).as_str());
                            let t = s.target().unwrap().as_string().unwrap();
                            //let accs: Vec<String> = t.split(",").into_iter().map(|i| i.into()).collect();                            
                            link.send_message(WalletMsg::SetAccount(JsValue::from_str(&t)))
                        })
                    ).expect("Cannot set accountsChanged listener");
                    
                }
                true
            },
            WalletMsg::GetChainId => {
                let provider = self.provider.clone();
                let link = ctx.link().clone();
                provider.request(
                    "eth_chainId".into(),
                    None,
                    Box::new(link),
                    Box::new(|m, l| {
                        let link = l.downcast_ref::<Scope<Wallet>>().unwrap();
                        match m {
                            Ok(cid) => {
                                let chain_id_s: String = serde_wasm_bindgen::from_value(cid).unwrap();
                                let c = i64::from_str_radix(
                                    &chain_id_s.trim_start_matches("0x"), 
                                    16
                                ).unwrap();
                                let c = Chain::from_id(c.try_into().unwrap());
                                link.send_message(WalletMsg::SetChainId(c))
                            },
                            Err(_e) => {
                                link.send_message(WalletMsg::SetError("No chain".to_string()))                              
                            }
                        }
                    })
                );
/* 
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
*/
                false
            },
            WalletMsg::SetChainId(chain_id) => {
                self.chain_id = Some(chain_id);
                true
            },
            WalletMsg::GetClientVersion => {
                let provider = self.provider.clone();
                let link = ctx.link().clone();
                provider.request(
                    "web3_clientVersion".into(),
                    None,
                    Box::new(link.clone()),
                    Box::new(|m, l| {
                        let link = l.downcast_ref::<Scope<Wallet>>().unwrap();
                        match m {
                            Ok(v) => {
                                let version: String = serde_wasm_bindgen::from_value(v).unwrap();
                                link.send_message(WalletMsg::SetClientVersion(version))
                            },
                            Err(err) => {
                                let err_f: String = serde_wasm_bindgen::from_value(err).unwrap();
                                link.send_message(WalletMsg::SetError(err_f))
                            }
                        }
                    })
                );
                false
            },
            WalletMsg::SetClientVersion(v) => {
                self.version = Some(v);
                true
            }

        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <table>
                if let Some(chain) = &self.chain_id {
                    /* <button
                        onclick={ctx.link().callback(|_| WalletMsg::Disconnect)}
                    >{"Disconnect"}</button> */
                    <tr>
                        <th>{"Chain"}</th>
                        <td>{chain}</td>
                    </tr>
                } else {
                    <button
                        onclick={ctx.link().callback(|_| WalletMsg::ConnectMetamask)}
                    >{"connect"}</button>
                }
                if let Some(account) = &self.account {
                    <tr>
                        <th>{"Account"}</th>
                        <td>{account}</td>
                    </tr>
                }
                if let Some(version) = &self.version {
                    <tr>
                        <th>{"version"}</th>
                        <td>{version}</td>
                    </tr>
                }
                
                if let Some(error) = &self.errors {
                    <tr>
                        <th>{"errors"}</th>
                        <td>{error}</td>
                    </tr>
                }                
            </table>
        )
    }
}
