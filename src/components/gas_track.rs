use yew::prelude::*;
use foundry_block_explorers::{Client, gas::GasOracle};
use std::sync::Arc;
use alloy_chains::Chain;
use gloo_timers::callback::Interval;
use crate::components::{
    gas_station::GasStation,
    lineal_chart::LinealChart,
};
use serde::Serialize;
use wasm_bindgen::JsValue;
use crate::helpers::format_gas;

pub enum GasTrackMsg {
    Pass,               // nothing happens
    FetchGas,
    GasOracle(GasOracle),
    SetError(String),
//    SetClient(Client),
//    StartInterval,
//    StopInterval,
} 

pub struct GasTrack {
    client: Option<Arc<Client>>,
    pub gas_history: Vec<GasOracle>,
    pub interval: Option<Interval>,
    pub errors: Option<String>,
}

impl Component for GasTrack {
    type Message = GasTrackMsg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        let api_key = "JCVMBDW3URWNNG7MFHA4FATVBRD9JMG9KN";
        if let Ok(client) = Client::new(Chain::mainnet(), api_key) {
            let ctx1 = ctx.link().clone();
            let handle = {
                Interval::new(4_000, move || ctx1.send_message(GasTrackMsg::FetchGas))
            };                
            Self {
                client: Some(Arc::new(client)),
                interval: Some(handle),
                gas_history: Vec::new(),
                errors: None,
            }
        } else {
            Self {
                client: None,
                interval: None,
                gas_history: Vec::new(),
                errors: None,
            }
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            /* TrackMsg::SetClient(client) => {
                ctx.link().send_message(TrackMsg::FetchGas);
                self.client = Some(Arc::new(client));
                true
            }, */
            GasTrackMsg::FetchGas => {
                let client = Arc::clone(&self.client.as_ref().unwrap());
                let last_block = match &self.gas_history.last() {
                    Some(go) => go.last_block,
                    None => 0
                };
                ctx.link().send_future(async move {
                    if let Ok(go) = client.gas_oracle()
                        .await
                        {                            
                            if go.last_block != last_block {
                                GasTrackMsg::GasOracle(go)
                            } else {
                                GasTrackMsg::Pass
                            }
                        } else {
                            GasTrackMsg::SetError("Missed block".to_string()) 
                        }
                    });
                false
            },
            GasTrackMsg::Pass => {
                false
            }
            GasTrackMsg::SetError(err) => {
                self.errors = Some(err);
                true
            },
            GasTrackMsg::GasOracle(go) => {
                self.gas_history.push(go);
                true   
            },
/* 
            TrackMsg::StartInterval => {
                let ctx1 = ctx.link().clone();
                let handle = {
                    Interval::new(4_000, move || ctx1.send_message(TrackMsg::FetchGas))
                };                
                self.interval = Some(handle);
                false
            },
            TrackMsg::StopInterval => {
                self.interval = None;
                true
            },
 */
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <main>
                if let Some(_client) = &self.client {
                    <>
                        <p>{"Connected"}</p>
/* 
                        if self.interval.is_none() {
                            <button onclick={ctx.link().callback(|_| TrackMsg::StartInterval)}>
                                {"Start interval"}
                            </button>
                        } else {
                            <button onclick={ctx.link().callback(|_| TrackMsg::StopInterval)}>
                                <div class={"spinner"}></div>
                                {"Stop interval"}                            
                            </button>                
                        } 
*/
                    </>
                } else {
                    <p>{"Not connected"}</p>
                }
                if let Some(gas) = &self.gas_history.last() {
                    <div class={"container"}>
                        <p>{"Block: "} {gas.last_block}</p>
                        <GasStation 
                            safe={gas.safe_gas_price}
                            fast={gas.fast_gas_price}
                            proposed={gas.propose_gas_price}
                        />
                        <LinealChart
                            list_to_display={self.lineal_plot_data()}
                        />
                        if let Some(err) = &self.errors {
                            <p>{err}</p>
                        }
                    </div>
                } 
            </main>
        )
    }
}

#[derive(Debug, Serialize)]
pub struct Tick {
    pub x: u64,         // block number
    pub y: usize,       // safe gas
    pub z: usize,       // fast gas
}

impl GasTrack {
    pub fn lineal_plot_data(&self) -> JsValue {
        let display: Vec<Tick> = self
            .gas_history
            .iter()
            .map(|b| {
                return Tick {
                    x: b.last_block,
                    y: format_gas(b.safe_gas_price),
                    z: format_gas(b.fast_gas_price),
                }
            })
            .collect();
        let parsed = serde_wasm_bindgen::to_value(&display).unwrap();
        parsed
    }

} 