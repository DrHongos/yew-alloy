use yew::prelude::*;
use crate::components::{
    gas_track::GasTrack,
    wallet::Wallet,
};

/* 
TODO: 
- create metamask SDK shims!        <

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
            <GasTrack />
        </main>
    }
}
