use crate::contexts::ethereum::UseEthereum;
use alloy_web::WalletType;
use yew::prelude::*;
use crate::helpers::log;

#[function_component(Wallet)]
pub fn wallet() -> Html {
    //let balance = use_state(|| U256::ZERO);
//    let nc_name = use_state(|| "ETH".to_string());
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );

    //let wc = use_state(|| false);

    let connected = ethereum.is_connected();
    let label = if connected {
        ethereum.main_account()
    } else {
        "Connect wallet".into()
    };
    let chain_label = ethereum.chain();
    //let balance = ethereum.balance();
/*     
    use_effect(move || {               // should have dependencies
        let nc = nc_name.clone();
        if let Some(chainp) = chain_label {
            let chain = NamedChain::try_from(chainp).unwrap();
            if let Some(nt) = chain.native_currency_symbol() {
                nc.set(nt.to_string());
            }
        }
    });
 */
//    let eth = ethereum.clone();
/*     let onclick = {
        let wc = wc.clone();
        Callback::from(move |_: MouseEvent| wc.set(!(*wc)))
    }; */
    let onclick_ethereum = {
        Callback::from(move |_: MouseEvent| {
            log("Connecting!");        
            if ethereum.is_connected() {
                ethereum.clone().disconnect();
            } else {
             //   if *wc {
             //       ethereum.clone().connect(WalletType::WalletConnect);
             //   } else {
                    ethereum.clone().connect(WalletType::Injected);
             //   }
            }
        })
    };
    
    html! {
        <div class="wallet-show">
            <button class={"button"} onclick={onclick_ethereum}>{label}</button>
            /* if !connected {
                <input type="checkbox" {onclick} disabled={!eth.walletconnect_available()}/ ><label>{"Wallet connect"}</label>
            } */        
            if let Some(cl) = chain_label {
                <div class="chain-display">
                    <div>{format!("{}", cl.named().expect("Chain with no name?"))}</div>
                </div>
            } 
        </div>
    }
}
