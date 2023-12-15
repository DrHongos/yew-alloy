use yew::prelude::*;
use alloy_primitives::U64;
use alloy_rpc_client::{ClientBuilder, RpcCall};
use std::borrow::Cow;
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;

#[function_component(ProviderTest)]
pub fn provider_test() -> Html {
    let infura = "https://mainnet.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1";

    let client = ClientBuilder::default().reqwest_http(infura.parse().unwrap());

    let get_block_number = Callback::from(move |_: MouseEvent| {
        let params: Cow<'static, _> = Cow::Owned(());        
        let req: RpcCall<_, Cow<'static, ()>, U64> = client.prepare("eth_blockNumber", params);
        spawn_local(async move {
            match req.await {
                Ok(d) => log(format!("{}", d).as_str()),
                _ => log("Error!")
            }
        })
    });    

    html!{
        <div class={"provider"}>
            <h3>{"Provider box"}</h3>
            <button onclick={get_block_number} class="button">{"block number"}</button>
        </div>
    }

}