use std::sync::Arc;
use alloy_primitives::Address;
use alloy_chains::Chain;
use alloy_web::{Ethereum, EthereumBuilder, EthereumError, Event, WalletType};
use serde::Serialize;
use yew::{platform::spawn_local, prelude::*};
//use crate::helpers::log;

#[derive(Clone, Debug)]
pub struct UseEthereum {
    pub ethereum: UseStateHandle<Ethereum>,
    pub connected: UseStateHandle<bool>,
    pub accounts: UseStateHandle<Option<Vec<Address>>>,
    pub chain_id: UseStateHandle<Option<u64>>,
    pub chain: UseStateHandle<Option<Chain>>,
    pub pairing_url: UseStateHandle<Option<String>>,
}

impl PartialEq for UseEthereum {
    fn eq(&self, other: &Self) -> bool {
        self.connected == other.connected
            && self.accounts == other.accounts
            && self.chain_id == other.chain_id
            && self.pairing_url == other.pairing_url
    }
}

impl UseEthereum {
    pub fn connect(&mut self, wallet_type: WalletType) {
        // We check if it is possible to connect
        let this = self.clone();
        if (*self.ethereum).is_available(wallet_type) {
            spawn_local(async move {
                let mut eth = (*this.ethereum).clone();
                let me = this.clone();
                if eth
                    .connect(
                        wallet_type,
                        Some(Arc::new(move |event| match event {
                            Event::ConnectionWaiting(url) => {
                                //debug!("{url}");
                                //log("Event: Connection waiting");
                                me.pairing_url.set(Some(url));
                            }
                            Event::Connected => {
                                //log("Event: Connected");
                                me.connected.set(true);
                                me.pairing_url.set(None)
                            }
                            Event::Disconnected => me.connected.set(false),
                            Event::ChainIdChanged(chain_id) => {
                                me.chain_id.set(chain_id);
                                //log(format!("Event: Chain changed {:#?}", chain_id).as_str());
                                if let Some(c) = chain_id {
                                    me.chain.set(Some(Chain::from_id(c)))
                                }
                            },
                            Event::AccountsChanged(accounts) => {
                                /* let accounts_parsed = accounts
                                    .unwrap()
                                    .into_iter()
                                    .map(|a| return Address::from_slice(a.as_bytes()))
                                    .collect(); */
                                //log(format!("Event: Account changed {:#?}", accounts).as_str());
                                me.accounts.set(accounts)
                            },
                        })),
                    )
                    .await
                    .is_ok()
                {
                    this.ethereum.set(eth);
                }
            });
        } else {
            println!("Error");
            //error!("This wallet type is unavailable!");
        }
    }
/* 
    pub fn provider(&self) -> Provider<Ethereum> {
        let eth = (*self.ethereum).clone();
        Provider::<Ethereum>::new(eth)
    }
 */
    pub fn disconnect(&mut self) {
        let mut eth = (*self.ethereum).clone();
        eth.disconnect();
        self.ethereum.set(eth);
        self.connected.set(false);
        self.chain.set(None);
    }

    pub fn is_connected(&self) -> bool {
        *self.connected
    }

    pub fn injected_available(&self) -> bool {
        (*self.ethereum).injected_available()
    }

    pub fn walletconnect_available(&self) -> bool {
        (*self.ethereum).walletconnect_available()
    }

    pub fn chain_id(&self) -> u64 {
        (*self.chain_id).unwrap_or(0)
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

    pub async fn sign_typed_data<T: Send + Sync + Serialize>(
        &self,
        data: T,
        from: &Address,
    ) -> Result<String, EthereumError> {    //<Signature, _>
        (*self.ethereum).sign_typed_data(data, from).await
    }
}

#[hook]
pub fn use_ethereum() -> UseEthereum {
    let mut builder = EthereumBuilder::new();

    if let Some(project_id) = std::option_env!("PROJECT_ID") {
        builder.walletconnect_id(project_id);
    }
    if let Some(rpc_url) = std::option_env!("RPC_URL") {
        builder.rpc_node(rpc_url);
    }
    let connected = use_state(move || false);
    let accounts = use_state(move || None as Option<Vec<Address>>);
    let chain_id = use_state(move || None as Option<u64>);
    let chain = use_state(move || None as Option<Chain>);
    let ethereum = use_state(move || builder.url("http://localhost").build());
    let pairing_url = use_state(move || None as Option<String>);

    UseEthereum {
        ethereum,
        connected,
        accounts,
        chain_id,
        chain,
        pairing_url,
    }
}
