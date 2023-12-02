use crate::contexts::ethereum::UseEthereum;
use ethers_web::WalletType;
use yew::prelude::*;

#[function_component(Wallet)]
pub fn wallet() -> Html {
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );

    let wc = use_state(|| false);

    let onclick = {
        let wc = wc.clone();
        Callback::from(move |_: MouseEvent| wc.set(!(*wc)))
    };

    let label = if ethereum.is_connected() {
        ethereum.main_account()
    } else {
        "Connect wallet".into()
    };
    let chain_label = ethereum.chain();
    let eth = ethereum.clone();
    let onclick_ethereum = {
        Callback::from(move |_: MouseEvent| {
            if ethereum.is_connected() {
                ethereum.clone().disconnect();
            } else {
                if *wc {
                    ethereum.clone().connect(WalletType::WalletConnect);
                } else {
                    ethereum.clone().connect(WalletType::Injected);
                }
            }
        })
    };
    html! {
        <>
            <input type="checkbox" {onclick} disabled={!eth.walletconnect_available()}/ ><label>{"Wallet connect"}</label>
            <button onclick={onclick_ethereum}>{label}</button>
            <p>{chain_label}</p>        
        </>
    }
}
