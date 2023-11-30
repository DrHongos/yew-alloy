use foundry_block_explorers::block_number::BlockNumber;
use ruint::aliases::U256;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use alloy_chains::Chain;
//use alloy_providers::Provider;
//use alloy_transport_http::Http;
use crate::components::wallet::html::Scope;
use eip1193::Provider as EIP1193Provider;
use crate::helpers::format_eth;

/* 
TODO:
- get native token (maybe contribute to alloy-chains?)
- exhaust EIP1193 tests
    - eth_suscribe (and unsuscribe) + event listener (message) from frontend!
- error handling
- check EIP6963
- parse alloy-provider?
*/

#[wasm_bindgen]
extern "C" {
// if i have a "Provider" like object, i can simply call "requests" (https://github.com/ZeroProphet/EIP1193_rs/blob/master/example/yewapp/)
//    #[wasm_bindgen(js_namespace=["window","ethereum"], js_name="request")]
//    pub fn request(m: &str);

    #[wasm_bindgen(js_namespace=["console"])]
    pub fn log(value: &str);    
}

pub enum WalletMsg {
    ConnectMetamask,
//    Disconnect,
    SetListeners,
    GetChainId,
    GetClientVersion,
    GetBalance(String),
    SetError(String),
    SetAccount(JsValue),
    SetChain(JsValue),
    SetClientVersion(String),
    SetBalance(U256),
} 

pub struct Wallet {
    account: Option<String>,
    chain: Option<Chain>,
    version: Option<String>,
    pub errors: Option<String>,
    balance: U256,
    // eip1193 method
    provider: EIP1193Provider,         // try to make it optional
}

impl Component for Wallet {
    type Message = WalletMsg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async move {
            WalletMsg::ConnectMetamask
        });
        let new_provider = EIP1193Provider::new();

        Self {
            account: None,
            chain: None,
            version: None,
            errors: None,
            provider: new_provider,
            balance: U256::from(0)
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WalletMsg::ConnectMetamask => {
                let provider = self.provider.clone();
                //log(&format!("Provider {:#?}", JSON::parse(&provider.this)));
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
                                link.send_message(WalletMsg::GetClientVersion);
                                link.send_message(WalletMsg::SetListeners);
                                link.send_message(WalletMsg::GetChainId);              
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
            WalletMsg::SetAccount(accs) => {
                let accounts: Vec<String> = serde_wasm_bindgen::from_value(accs).unwrap();
                if let Some(acc) = accounts.first() {
                    let account = acc.to_string();
                    self.account = Some(account.clone());
                    // below are all updates of the connected account
                    ctx.link().send_message(WalletMsg::GetBalance(account));
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
                                link.send_message(WalletMsg::SetChain(cid))
                            },
                            Err(_e) => {
                                link.send_message(WalletMsg::SetError("No chain".to_string()))                              
                            }
                        }
                    })
                );
                false
            },
            WalletMsg::SetListeners => {                    
                let provider = self.provider.clone();
                let link = ctx.link().clone();
                let link2 = ctx.link().clone(); 

                provider.clone().on(
                    "accountsChanged".into(), 
                    Box::new(
                        move |s| {
                            link.send_message(WalletMsg::SetAccount(s.into()))
                        })
                );
                provider.on(
                    "chainChanged".into(),
                    Box::new(
                        move |s| {
                            link2.send_message(WalletMsg::SetChain(s.into()))
                        })
                );
                
                false
            },
            WalletMsg::SetChain(chain_id) => {
                let chain_id_s: String = serde_wasm_bindgen::from_value(chain_id).unwrap();
                let c = i64::from_str_radix(
                    &chain_id_s.trim_start_matches("0x"), 
                    16
                ).unwrap();
                let c = Chain::from_id(c.try_into().unwrap());
                self.chain = Some(c);
                ctx.link().send_message(WalletMsg::GetBalance(self.account.clone().unwrap()));
                true
            },
            WalletMsg::GetBalance(account) => {
                let provider = self.provider.clone();
                let link = ctx.link().clone();
                log("Check balance");
                provider.request(   
                    "eth_getBalance".into(),
                    Some(vec![
                        account, 
                        //BlockNumber::Latest.to_string(),
                        ]),
                        Box::new(link),
                        Box::new(|m, l| {
                            let linkk = l.downcast_ref::<Scope<Wallet>>().unwrap();
                            match m {
                            Ok(b) => {
                                let bal: U256 = serde_wasm_bindgen::from_value(b).expect("error parsing balance");
                                log(format!("Balance: {}", bal).as_str());
                                linkk.send_message(WalletMsg::SetBalance(bal))
                            },
                            Err(err) => {
                                let err_f: String = serde_wasm_bindgen::from_value(err).unwrap();
                                log(format!("Error: {}", err_f).as_str());
                                linkk.send_message(WalletMsg::SetError(err_f))
                            }
                        }
                    })
                );
                false
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
            },
            WalletMsg::SetBalance(b) => {
                self.balance = b;
                true
            }

        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <table>
                if let Some(account) = &self.account {
                    <tr>
                    <th>{"Account"}</th>
                    <td>{account}</td>
                    </tr>
                }
                if let Some(chain) = &self.chain {
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
                <tr>
                    <th>{"Balance"}</th>
                    <td>{format_eth(self.balance)} {"   ETH(?)"}</td>
                </tr>
                <hr />                
                //<tr><th>{"Is metamask"}</th><td>{self.provider.is_metamask}</td></tr>
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
