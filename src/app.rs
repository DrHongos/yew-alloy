use yew::prelude::*;
use crate::contexts::provider::EthereumContextProvider;
use crate::contexts::ethereum::UseEthereum;
use crate::components::{
//    gas_track::GasTrack,
    wallet::Wallet,
//    button_sign::SignatureButton,
    rpc_tester::RpcTest,
//    printer_test::PrinterTest,
};
/* 
TODO: 
- guardar ETHERSCAN_API_KEY in localstorage (web-sys)

*/
#[function_component(App)]
pub fn app() -> Html {

    html! {
        <main>
            <EthereumContextProvider>
                <div class="header" id="Header">
                    <b>{ "AlloYew" }</b>
                    <div class="superLogo">
                        <a href={"https://github.com/alloy-rs"}  target={"blank"}>
                        <img class="logo" src="https://avatars.githubusercontent.com/u/128098468?s=200&v=4" alt="Yew logo" />
                        </a>
                        <i class="heart" />
                        <a href={"https://yew.rs/"} target={"blank"}>
                        <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
                        </a>
                    </div>
                    <Wallet />
                </div>
                <div class="content">
                    <Tools />            
                </div>
            </EthereumContextProvider>
        </main>
    }
}
        
#[function_component(Tools)]
pub fn tools() -> Html {
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let tools = match ethereum.is_connected() {
        true => html!{
            <RpcTest />
        },
        false => html!{<p>{"Connect to window.ethereum to see tools"}</p>},
    };
    
    html!{
        {tools}        
    }
}