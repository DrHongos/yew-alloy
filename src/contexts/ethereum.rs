use alloy_primitives::Address;
use alloy_chains::Chain;
use alloy_web::{
    BrowserTransport, 
    WalletType, 
    provider::Provider,
    Event
};
use yew::{platform::spawn_local, prelude::*};
use crate::helpers::log;
use std::sync::Once;
use std::sync::Arc;
use foundry_block_explorers::Client;

#[derive(Clone, Debug)]
pub struct UseEthereum {
    pub provider : UseStateHandle<Option<Provider<BrowserTransport>>>,
    pub connected: UseStateHandle<bool>,
    pub accounts: UseStateHandle<Option<Vec<Address>>>,
    pub chain: UseStateHandle<Option<Chain>>,
    //pub pairing_url: UseStateHandle<Option<String>>,
    pub etherscan_client: UseStateHandle<Option<Client>>,
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
        let this_too = this.clone();
        let btrans = BrowserTransport::new(); // not used
        if btrans.is_available(wallet_type) {
            spawn_local(async move {
                match Provider::new(
                    wallet_type,
                    Some(Arc::new(move |event| match event {
                        Event::ConnectionWaiting(_url) => {
                            log("Event: Connection waiting");
                        }
                        Event::Connected => {
                            log("Event: Connected");
                        }
                        Event::Disconnected => {
                            log("Event: Disconnected");   
                        },
                        Event::ChainIdChanged(chain_id) => {
                            log(format!("Event: chainidchanged {:#?}", chain_id).as_str());
                            this_too.chain.set(Some(Chain::from_id(chain_id.unwrap())));
                            },
                        Event::AccountsChanged(accounts) => {
                            log(format!("Event: Account changed {:#?}", accounts).as_str());
                            if let Some(acc) = accounts {
                                if acc.len() == 0 { 
                                    this_too.accounts.set(None);
                                    this_too.chain.set(None);
                                    this_too.connected.set(false);
                                    this_too.provider.set(None); 
                                } else {
                                    this_too.accounts.set(Some(acc));
                                }
                            } 
                    },
                })),
                ).await {
                    Some(provider) => {
                        log(format!("Setting provider {:#?}", provider).as_str());
                        //let acc = provider.from.clone();
                        let acc = provider.get_accounts().await.unwrap();
                        let c = provider.get_chain_id().await.unwrap();
                        let c: u64 = c.try_into().unwrap_or(1);
                        this.accounts.set(Some(acc));
                        let chain = Chain::from_id(c);
                        this.chain.set(Some(chain.clone()));
                        this.connected.set(true);
                        this.provider.set(Some(provider));
                        if let Ok(Some(local_storage)) = web_sys::window().expect("No window?").local_storage() {
                            if let Ok(Some(value)) = local_storage.get_item("etherscan_api_key") {
                                log(format!("local storage api key {}", value).as_str());
                                this.etherscan_client.set(Some(Client::new(chain, value).expect("Cannot create client")));
                            }
                        }
                    },
                    None => {log("Rejected connection")}
                }
            }
        );
        } else {
            println!("Error");
        }
    }

    pub fn disconnect(&mut self) {
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
            .expect("empty")
            .first()
            .unwrap_or(&Address::ZERO)
    }

    pub fn main_account(&self) -> String {
        self.account().to_string()
    }
    // function used to resume on refreshes
    pub async fn resume(&mut self) {
        match Provider::<BrowserTransport>::resume().await {
            Ok(r) => {
                if r {
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
    let etherscan_client = use_state(move || None as Option<Client>);

    let e = UseEthereum {
        //        ethereum,
        provider,
        connected,
        accounts,
        chain,
        etherscan_client,
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
