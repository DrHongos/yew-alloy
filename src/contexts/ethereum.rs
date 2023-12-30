use std::sync::Arc;
use alloy_primitives::Address;
use alloy_chains::Chain;
use alloy_web::{
    BrowserTransport, 
    builder::BrowserTransportBuilder, 
    Event, 
    WalletType, 
    Provider,
};
use yew::{platform::spawn_local, prelude::*};
use crate::helpers::log;

#[derive(Clone, Debug)]
pub struct UseEthereum {
    pub ethereum: UseStateHandle<BrowserTransport>,
    pub provider : UseStateHandle<Option<Provider<BrowserTransport>>>,
    pub connected: UseStateHandle<bool>,
    pub accounts: UseStateHandle<Option<Vec<Address>>>,
    pub chain: UseStateHandle<Option<Chain>>,
    pub pairing_url: UseStateHandle<Option<String>>,
    //pub chain_id: UseStateHandle<Option<u64>>,
}

impl PartialEq for UseEthereum {
    fn eq(&self, other: &Self) -> bool {
        self.connected == other.connected
            && self.accounts == other.accounts
            && self.chain == other.chain
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
                //let other = me.clone();
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
                                me.chain.set(Some(Chain::from_id(chain_id.expect("No chain id"))));
                                /* let c = other.clone();
                                spawn_local(async move {
                                    c.clone().get_native_balance().await
                                }); */
                                if let Some(c) = chain_id {
                                    me.chain.set(Some(Chain::from_id(c)))
                                }
                            },
                            Event::AccountsChanged(accounts) => {
                                //log(format!("Event: Account changed {:#?}", accounts).as_str());
                                /* let c = other.clone();
                                spawn_local(async move {
                                    c.clone().get_native_balance().await
                                }); */
                                me.accounts.set(accounts)
                            },
                        })),
                    )
                    .await
                    .is_ok()
                {
                    this.ethereum.set(eth.clone());
                    let provider = Provider::new(eth).await;
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
/* 
    // does not get the values in initial launch
    pub async fn get_native_balance(&self, account: String) {
        log(format!("calling {}", account).as_str());
        //let account = self.account();
        let param1: Cow<'static, String> = Cow::Owned(account);        
        let param2: Cow<'static, String> = Cow::Owned("latest".to_string());
        let params = vec![param1, param2];
        if let Ok(client) = self.client() {
            let req_bal: RpcCall<_, Vec<Cow<_>>, U256> = client.prepare("eth_getBalance", params);
            let balance = req_bal.await.expect("Could not get balance"); 
            self.balance.set(Some(balance));
        }
    }
 */

    pub fn disconnect(&mut self) {
        let mut eth = (*self.ethereum).clone();
        eth.disconnect();
        self.ethereum.set(eth);
//        self.client.set(None);
        self.provider.set(None);
        self.connected.set(false);
        self.chain.set(None);
    }

    pub fn is_connected(&self) -> bool {
        *self.connected
    }

    pub fn injected_available(&self) -> bool {
        (*self.ethereum).injected_available()
    }
/* 
    pub fn walletconnect_available(&self) -> bool {
        (*self.ethereum).walletconnect_available()
    }
 */
    pub fn chain(&self) -> Option<Chain> {
        *self.chain
    }
/* 
    pub fn balance(&self) -> Option<U256> {
        *self.balance
    }
 */
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
/* 
    pub async fn sign_typed_data<T: Send + Sync + Serialize>(
        &self,
        data: T,
        from: &Address,
    ) -> Result<String, EthereumError> {    //<Signature, _>
        (*self.ethereum).sign_typed_data(data, from).await
    }
*/
}

#[hook]
pub fn use_ethereum() -> UseEthereum {
    let mut builder = BrowserTransportBuilder::new();

    if let Some(project_id) = std::option_env!("PROJECT_ID") {
        builder.walletconnect_id(project_id);
    }
    if let Some(rpc_url) = std::option_env!("RPC_URL") {
        builder.rpc_node(rpc_url);
    }
    let connected = use_state(move || false);
    let accounts = use_state(move || None as Option<Vec<Address>>);
    let chain = use_state(move || None as Option<Chain>);
    let ethereum = use_state(move || builder.url("http://localhost").build());
    let pairing_url = use_state(move || None as Option<String>);
    let provider = use_state(move || None as Option<Provider<BrowserTransport>>);

    UseEthereum {
        ethereum,
        provider,
        connected,
        accounts,
        chain,
        pairing_url,
    }
}
