//#![allow(non_snake_case)]
//use yew::prelude::*;
//use ethers::{prelude::*, utils::format_units};
//use std::sync::Arc;
/* 
use gloo_timers::callback::Interval;
use wasm_bindgen::prelude::*;
use serde::Serialize;
use crate::{components::{
    lineal_chart::LinealChart,
    last_block::LastBlock,
    history_blocks::HistoryBlocks,
}};
 */
//const API_MAINNET_KEY: &str = dotenv!("WSS_KEY_MAINNET");
/* 
pub enum TrackerMsg {
    SetClient(Provider<Ws>),
    FetchLastBlock,
    SetLastBlock(Block<H256>),    
    SetError(String),
    StartInterval,
    StopInterval,
}
pub struct Tracker {
    client: Option<Arc<Provider<Ws>>>,
    last_block: Option<Block<H256>>,    
    interval: Option<Interval>,
    error: Option<String>,
    list_to_display: Vec<Block<H256>>,
}

impl Component for Tracker {
    type Message = TrackerMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match Ws::connect(API_MAINNET_KEY).await {
                Ok(ws) => {
                    let provider = Provider::new(ws);
//                    let client_ver = provider.client_version().await.unwrap();
//                    log::info!("Provider: {}",client_ver);
                    TrackerMsg::SetClient(provider)
                },
                Err(err) => {
                    log::error!("Error in connection to provider: {:?}", err);
                    TrackerMsg::SetError(err.to_string())
                }
            } 
        });
        Self {
            client: None,
            last_block: None,
            error: None,
            interval: None,            
            list_to_display: Vec::new(),
            //streamer: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {            
            TrackerMsg::FetchLastBlock => {
                let client = Arc::clone(&self.client.as_ref().unwrap());
                ctx.link().send_future(async move {
                    match client.get_block(BlockNumber::Latest).await {
                        Ok(block) => {
                            let block = block.unwrap();
                            TrackerMsg::SetLastBlock(block)
                        },
                        Err(err) => TrackerMsg::SetError(err.to_string())
                    }
                    });
                false
            }
            TrackerMsg::StartInterval => {
                let ctx1 = ctx.link().clone();
                let handle = {
                    // blocks are 12s apart. Having 6 to update faster the list and maybe catch something?
                    Interval::new(6_000, move || ctx1.send_message(TrackerMsg::FetchLastBlock))
                };                
                self.interval = Some(handle);
                log::info!("Started interval");
                true
            }
            TrackerMsg::StopInterval => {
                //handler.cancel(); // HOW TO DEREFERENCE? IS THIS THE GWEI?
                self.interval = None;                // this works!
                //self.mempool_interval = None;
                log::info!("Stopped interval");
                true
            }
            TrackerMsg::SetClient(provider) => {
                self.client = Some(Arc::new(provider));
                ctx.link().send_message(TrackerMsg::FetchLastBlock); // first fetch
//                ctx.link().send_message(TrackerMsg::StartInterval); // auto-start
                true
            }
            TrackerMsg::SetLastBlock(bn) => {                            
                // moves last_block into list and updates the plot                
                if let Some(pb) = self.last_block.clone() {
                    // check that last_block is not in list
                    if !self.list_to_display.contains(&pb) {
                        self.list_to_display.push(pb);
                    }
                }
                self.last_block = Some(bn);
                true
            }
            TrackerMsg::SetError(err) => {
                self.error = Some(err);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main class="container">
                if let Some(error) = &self.error {
                    <p>{ format!("Error: {:?}", error) }</p>
                }
                <table class="table">
                    if !self.client.is_none() {
                        if self.interval.is_none() {
                            <button onclick={ctx.link().callback(|_| TrackerMsg::StartInterval)}>
                                {"Start interval"}
                            </button>
                        } else {
                            <button onclick={ctx.link().callback(|_| TrackerMsg::StopInterval)}>
                                <div class={"spinner"}></div>
                                {"Stop interval"}                            
                            </button>                
                        }
                    }
                    <tr>
                        <th> {"Last block"}</th>
                        <th> {"Previously, on mainnet.."}</th>
                        if self.list_to_display.len() != 0 {
                            <th>{"Plots are cool"}</th>                    
                        }    
                    </tr>
                    <tr>
                    <td>
                        if let Some(last_block) = &self.last_block {
                            <LastBlock
                                last_block={last_block.clone()}
                            />
                        } else {
                            <p>{"Fetching.."}</p>
                        }
                    </td>
                    <td>
                        <HistoryBlocks
                            blocks = {self.list_to_display.clone()} 
                        />
                    </td>
                    if self.list_to_display.len() != 0 {
                        <LinealChart
                            list_to_display = {self.lineal_plot_data()}
                        />                    
                    }
                    </tr>
                </table>
                if !self.client.is_none() {
                    <p>{"Provider (WS) connected"}</p>
                }
            </main>
        }
    }
    
}

#[derive(Debug, Serialize)]
pub struct Tick {
    pub x: u64,
    pub y: String, // hardcoded to base_fee_per_gas attribute
}

impl Tracker {
    pub fn lineal_plot_data(&self) -> JsValue {
        let display: Vec<Tick> = self
            .list_to_display
            .iter()
            .map(|b| {
                return Tick {
                    x: b.timestamp.as_u64(), // convert to datetime?
                    y: format_units(b.base_fee_per_gas.unwrap_or_default(), 9).unwrap(),
                }
            })
            .collect();
        let parsed = serde_wasm_bindgen::to_value(&display).unwrap();
        parsed
    }

} */