use std::sync::Arc;
use alloy_primitives::Address;
use alloy_chains::Chain;
use alloy_web::{
    BrowserTransport, 
    Event, 
    WalletType, 
    provider::Provider,
};
use yew::{platform::spawn_local, prelude::*};
use crate::helpers::log;
use std::sync::Once;

#[derive(Clone, Debug)]
pub struct UseEthereum {
//    pub ethereum: UseStateHandle<BrowserTransport>,
    pub provider : UseStateHandle<Option<Provider<BrowserTransport>>>,
    pub connected: UseStateHandle<bool>,
    pub accounts: UseStateHandle<Option<Vec<Address>>>,
    pub chain: UseStateHandle<Option<Chain>>,
    //pub pairing_url: UseStateHandle<Option<String>>,
    //pub chain_id: UseStateHandle<Option<u64>>,
}

impl PartialEq for UseEthereum {
    fn eq(&self, other: &Self) -> bool {
        self.connected == other.connected
            && self.accounts == other.accounts
            && self.chain == other.chain
            //&& self.pairing_url == other.pairing_url
    }
}

impl UseEthereum {
    pub fn connect(&mut self, wallet_type: WalletType) {
        // We check if it is possible to connect
        let this = self.clone();
        let mut btrans = BrowserTransport::new(); 
        if btrans.is_available(wallet_type) {
            spawn_local(async move {
                let me = this.clone();
                if btrans
                    .connect(
                        wallet_type,
                        Some(Arc::new(move |event| match event {
                            Event::ConnectionWaiting(_url) => {
                                //debug!("{url}");
                                //log("Event: Connection waiting");
                                //me.pairing_url.set(Some(url));
                            }
                            Event::Connected => {
                                //log("Event: Connected");
                                me.connected.set(true);
                                //me.pairing_url.set(None)
                            }
                            Event::Disconnected => me.connected.set(false),
                            Event::ChainIdChanged(chain_id) => {
                                me.chain.set(Some(Chain::from_id(chain_id.expect("No chain id"))));
                                if let Some(c) = chain_id {
                                    me.chain.set(Some(Chain::from_id(c)))
                                }
                            },
                            Event::AccountsChanged(accounts) => {
                                //log(format!("Event: Account changed {:#?}", accounts).as_str());
                                me.accounts.set(accounts)
                            },
                        })),
                    )
                    .await
                    .is_ok()
                {
                    let provider = Provider::new(btrans).await;
                    log(format!("Setting provider {:#?}", provider).as_str());
                    let acc = provider.from.clone();
                    let c = provider.chain.clone();
                    this.accounts.set(Some(acc));
                    this.chain.set(Some(c));
                    this.provider.set(Some(provider));
                }
            });
        } else {
            println!("Error");
        }
    }

    pub fn disconnect(&mut self) {
        //let mut eth = (*self.ethereum).clone();
        //eth.disconnect();
        //self.ethereum.set(eth);
//        self.client.set(None);
        self.provider.set(None);
        self.connected.set(false);
        self.chain.set(None);
    }

    pub fn is_connected(&self) -> bool {
        *self.connected
    }
    pub fn chain(&self) -> Option<Chain> {
        *self.chain
    }
    pub fn account(&self) -> Address {
        *self
            .accounts
            .as_ref()
            .and_then(|a| a.first())
            .unwrap_or(&Address::ZERO)
    }

    pub fn main_account(&self) -> String {
        self.accounts
            .as_ref()
            .and_then(|a| a.first())
            .unwrap_or(&Address::ZERO)
            .to_string()
    }
    // function used to resume on refreshes
    pub async fn resume(&mut self) {
        match Provider::<BrowserTransport>::resume().await {
            Ok(p) => {
                if let Some(_p) = p {
                    // its is connected!
                    self.connect(WalletType::Injected);
                } else {
                    log("disconnected");
                }
            },
            Err(e) => {
                log(format!("Error {:#?}", e).as_str());
            }
        }
        
    }
}

#[hook]
pub fn use_ethereum() -> UseEthereum {  // check SuspensionResult protocol
    /* 
    to implement wallet-connect (from ethers-web)
    let mut builder = BrowserTransportBuilder::new();
    if let Some(project_id) = std::option_env!("PROJECT_ID") {
        builder.walletconnect_id(project_id);
    }
    if let Some(rpc_url) = std::option_env!("RPC_URL") {
        builder.rpc_node(rpc_url);
    } 
    */
    //    let ethereum = use_state(move || builder.url("http://localhost").build());
    //    let pairing_url = use_state(move || None as Option<String>);
    
    
    let provider = use_state(move || None as Option<Provider<BrowserTransport>>);
    let connected = use_state(move || false);
    let accounts = use_state(move || None as Option<Vec<Address>>);
    let chain = use_state(move || None as Option<Chain>);
    
    let e = UseEthereum {
        //        ethereum,
        provider,
        connected,
        accounts,
        chain,
        //        pairing_url,
    };
    // this clones the provider and checks if its connected, if so, it re-connects
    let t = e.clone();
    static START: Once = Once::new();
    START.call_once(|| {
        spawn_local(async move {
            t.clone().resume().await;
        }); 
    });
    
    e 
}
