use yew::prelude::*;
use crate::components::{
//    gas_track::GasTrack,
    wallet::Wallet,
};

/* 
TODO: 
- move metamask to a context https://yew.rs/docs/concepts/contexts
- create metamask SDK shims
    - un/suscribe
    https://docs.metamask.io/wallet/reference/eth_subscribe/
        - newheads
        - logs
        - pending transactions
        - syncing

    - would be great if we use foundry-block-explorers (or eth_getCode) to GET SOME CONTRACT CODE
        then detect its events
        and offer subscription directly

- guardar ETHERSCAN_API_KEY in localstorage (web-sys)
- agregar mas funciones
-   foundry_block_explorer
    - follow accounts, tokens, nfts, etc
-   alloy
    - Provider!!
*/
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Ahoy Alloy!" }</h1>
            <a href="https://github.com/alloy-rs">{"github"}</a>
        
            <Wallet />
            /* <GasTrack /> */
        </main>
        }
}
