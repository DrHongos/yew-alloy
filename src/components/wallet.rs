use crate::contexts::ethereum::UseEthereum;
use alloy_web::WalletType;
use yew::prelude::*;

#[function_component(Wallet)]
pub fn wallet() -> Html {
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );

    let connected = ethereum.is_connected();
    let label = if connected {
        ethereum.main_account()
    } else {
        "Connect wallet".into()
    };
    let chain_label = ethereum.chain();

    let onclick_ethereum = {
        Callback::from(move |_: MouseEvent| {
            if ethereum.is_connected() {
                ethereum.clone().disconnect();
            } else {
                ethereum.clone().connect(WalletType::Injected);
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
