use yew::prelude::*;
use foundry_block_explorers::{Client, gas::GasOracle};
use std::sync::Arc;
use alloy_chains::Chain;
use ruint::Uint;
use gloo_timers::callback::Interval;

/* 
TODO: 
- agregar plot?
- listar gas historico
- connect wallet
- agregar mas funciones
-   foundry_block_explorer
-   alloy
    - Provider!!
*/

pub enum TrackMsg {
    FetchGas,
    GasOracle(GasOracle),
    SetError(String),
//    SetClient(Client),
//    StartInterval,
//    StopInterval,
} 

pub struct Track {
    client: Option<Arc<Client>>,
    pub gas: Option<GasOracle>,
    pub interval: Option<Interval>,
    pub errors: Option<String>,
}

impl Component for Track {
    type Message = TrackMsg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        let api_key = "JCVMBDW3URWNNG7MFHA4FATVBRD9JMG9KN";
        if let Ok(client) = Client::new(Chain::mainnet(), api_key) {
            let ctx1 = ctx.link().clone();
            let handle = {
                Interval::new(4_000, move || ctx1.send_message(TrackMsg::FetchGas))
            };                
            Self {
                client: Some(Arc::new(client)),
                gas: None,
                interval: Some(handle),
                errors: None,
            }
        } else {
            Self {
                client: None,
                gas: None,
                interval: None,
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
            TrackMsg::FetchGas => {
                let client = Arc::clone(&self.client.as_ref().unwrap());
                ctx.link().send_future(async move {
                    if let Ok(go) = client.gas_oracle()
                        .await
                        {
                            TrackMsg::GasOracle(go)
                        } else {
                            TrackMsg::SetError("Missed block".to_string()) 
                        }
                    });
                false
            },
            TrackMsg::SetError(err) => {
                self.errors = Some(err);
                true
            },
            TrackMsg::GasOracle(go) => {
                self.gas = Some(go);
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
                if let Some(gas) = &self.gas {
                    <div class={"container"}>
                        <p>{"Block: "} {gas.last_block}</p>
                        <p>{"Safe: "} {gas.safe_gas_price.checked_div(Uint::from(1000000000))}</p>
                        <p>{"Proposed: "} {gas.propose_gas_price.div_ceil(Uint::from(1000000000))}</p>
                        <p>{"Fast: "} {gas.fast_gas_price.div_ceil(Uint::from(1000000000))}</p>
                        if let Some(err) = &self.errors {
                            <p>{err}</p>
                        }
                    </div>
                } 
            </main>
        )
    }
}
