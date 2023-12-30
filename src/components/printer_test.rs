use yew::prelude::*;
use alloy_json_rpc::{Request, RequestMeta, RequestPacket, Id, SerializedRequest};
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use crate::contexts::ethereum::UseEthereum;
use wasm_bindgen::prelude::*;
use alloy_web::WebClient;
use std::sync::Arc;

// testing
#[wasm_bindgen(inline_js = "export function log_provider_js() {console.log(window.ethereum)}")]
extern "C" {
    #[wasm_bindgen]
    fn log_provider_js();
}

// implement nrequest() to test it 

#[function_component(PrinterTest)]
pub fn printer_test() -> Html {

    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let eth = Arc::new(ethereum.ethereum.clone());

    let onlog = {
        //let ethereum = ethereum.clone();
        Callback::from(move |_: MouseEvent| {
            spawn_local(async move {
                log_provider_js();
            })
        })
    };

    let on_send = {        
        let eth1 = eth.clone();
        Callback::from(move |_: MouseEvent| {
            let request = Request { 
                meta: RequestMeta { 
                    method: "eth_chainId", 
                    id: Id::Number(0) 
                }, 
                params: Vec::<String>::new() 
            };
            let req = SerializedRequest::try_from(request).unwrap();
            let eth = eth1.clone();
            spawn_local(async move {
                let t = eth.send(req).await;
                log(format!("Made it! {:#?}", t).as_str());
            })
        })
    };
    
    let eth2 = eth.clone();
    let on_send_packet = {
        Callback::from(move |_: MouseEvent| {
            let request = Request { 
                meta: RequestMeta { 
                    method: "eth_chainId",  
                    id: Id::Number(0) 
                }, 
                params: Vec::<String>::new()//String::from("latest") 
            };
            let req = SerializedRequest::try_from(request).unwrap();
            let eth = eth2.clone();
            let reqp = RequestPacket::Single(req);
            spawn_local(async move {
                let t = eth.send_packet(reqp).await;
                log(format!("Made it! {:#?}", t).as_str());
            })
        })
    };

    let eth3 = eth.clone();
    let on_send_packet_batch = {
        Callback::from(move |_: MouseEvent| {
            let eth = eth3.clone();
            let request = Request { 
                meta: RequestMeta { 
                    method: "eth_chainId", //"eth_requestAccounts", 
                    id: Id::Number(0) 
                }, 
                params: Vec::<String>::new() 
            };
            let req = SerializedRequest::try_from(request).unwrap();

            let request2 = Request { 
                meta: RequestMeta { 
                    method: "web3_clientVersion", 
                    id: Id::Number(1) 
                }, 
                params: Vec::<String>::new() 
            };
            let req2 = SerializedRequest::try_from(request2).unwrap();

            /*             
            let request3 = Request { 
                meta: RequestMeta { 
                    method: "eth_gasPrice", 
                    id: Id::Number(1) 
                }, 
                params: String::new() 
            };
            let req3 = SerializedRequest::try_from(request3).unwrap();
            */
            let request4 = Request { 
                meta: RequestMeta { 
                    method: "eth_blockNumber", 
                    id: Id::Number(4) 
                }, 
                params: Vec::<String>::new() 
            };
            let req4 = SerializedRequest::try_from(request4).unwrap();
            let reqp = RequestPacket::Batch(vec![req, req2/* req3 */, req4 ]);
            spawn_local(async move {
                let t = eth.send_packet(reqp).await;
                log(format!("Made it! {:#?}", t).as_str());
            })
        })
    };

    html!{
        <div class={"provider"}>
        if ethereum.is_connected() {
            <p>{"basic"}</p>
            <button onclick={onlog} class={"button"} >{"Log"}</button>
            <hr />
            <p>{"send serializedRequest"}</p>
            <button onclick={on_send} class={"button"} >{"send(chainId)"}</button>
            <hr />
            <p>{"send packets"}</p>
            <hr />
            <p>{"Single"}</p>
            <button onclick={on_send_packet} class={"button"} >{"send_packet(Single::chainId)"}</button>
            <hr />
            <p>{"Batched"}</p>
            <button onclick={on_send_packet_batch} class={"button"} >{"send_packet(Batch::(chainId, version))"}</button>
        }
        </div>
    }
}