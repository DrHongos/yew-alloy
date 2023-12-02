use yew::prelude::*;
use crate::contexts::provider::EthereumContextProvider;
use crate::components::{
//    gas_track::GasTrack,
    wallet::Wallet,//       Re-do w/ context
};

/* 
TODO: 
- guardar ETHERSCAN_API_KEY in localstorage (web-sys)
- create Transport -> Provider for browser wallets -> wasm dapps

*/
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <EthereumContextProvider>
            <h1>{ "Ahoy Alloy!" }</h1>
            <div class="superLogo">
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <i class="heart" />
            <img class="logo" src="https://avatars.githubusercontent.com/u/128098468?s=200&v=4" alt="Yew logo" />
            </div>
            <a href="https://github.com/alloy-rs">{"github"}</a>
        
            <Wallet />
            /* <GasTrack /> */
            </EthereumContextProvider>
        </main>
        }
}
