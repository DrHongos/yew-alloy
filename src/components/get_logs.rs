use yew::prelude::*;
use crate::contexts::ethereum::UseEthereum;
use alloy_rpc_types::{
    Filter, 
    //Log,
};
use wasm_bindgen_futures::spawn_local;
use crate::helpers::log;
use std::ops::Deref;
use crate::components::filter::FilterCreator;

/* 
TODO:
    - finish filter  
    - handle error cases
    - display info

*/

#[function_component(GetLogs)]
pub fn get_logs() -> Html {
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let client = ethereum.provider.clone();
    let filter = use_state(|| Filter::new());
//    let logs = use_state(|| Vec::<Log>::new());
    
    let on_filter: Callback<Filter> = {
        let f = filter.clone();
        let client = client.clone();
        Callback::from(move |inp: Filter| {
            f.set(inp.clone());
            let client = client.clone();
            let inp = inp.clone();
            spawn_local(async move {
                if let Some(client) = client.deref() {
                    match client.get_logs(inp).await {
                        Ok(d) => log(format!("latest logs are {:#?}", d).as_str()),
                        _ => log("Error!")
                    }     
                } else { log("not connected") }
            })
        })
    };

    html!{
        <div class={"getCode"}>
            if ethereum.is_connected() {
                <FilterCreator 
                    on_filter={on_filter}
                />
                // collect logs?
            }
        </div>
    }

}