use wasm_bindgen::prelude::*;
// HELPERS
use ruint::{
    Uint,
    aliases::U256
};

pub fn format_gas(val: U256) -> usize {
    match val.checked_div(Uint::from(1000000000)) {
        Some(v) => v.try_into().unwrap(),
        None => 0
    }
}

pub fn format_eth(val: U256) -> f64 {
    let v: f64 = val.into();
    v/1000000000000000000f64        // parses to 18 decimals
}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace=["console"])]
    pub fn log(value: &str);    
}