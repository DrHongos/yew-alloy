//! EIP-1193 transport
//!
//! This transport lets you use the library inside a browser to interact with
//! EIP-1193 providers like MetaMask. It's intended for use with Rust's
//! WebAssembly target.

//use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use js_sys::Promise;
use wasm_bindgen_futures::spawn_local;
use std::any::Any;
//use eip1193::RequestMethod;

#[wasm_bindgen]
// Rustfmt removes the 'async' keyword from async functions in extern blocks. It's fixed
// in rustfmt 2.
#[rustfmt::skip]
extern "C" {
    #[derive(Clone, Debug)]
    /// An EIP-1193 provider object. Available by convention at `window.ethereum`
    pub type Provider;

    #[wasm_bindgen(catch, method)]
    async fn request(_: &Provider, args: RequestArguments) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method)]
    pub fn on(_: &Provider, eventName: &str, listener: &Closure<dyn FnMut(JsValue)>);

    #[wasm_bindgen(method, js_name = "removeListener")]
    pub fn removeListener(_: &Provider, eventName: &str, listener: &Closure<dyn FnMut(JsValue)>);
}

#[wasm_bindgen(inline_js = "export function get_provider_js() {return window.ethereum}")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn get_provider_js() -> Result<Option<Provider>, JsValue>;
}

// test this property
#[wasm_bindgen(inline_js = "export function get_is_metamask() {return window.ethereum.isMetamask}")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn get_is_metamask() -> Result<Option<bool>, JsValue>;
}

impl Provider {
    /// Get the provider at `window.ethereum`.
    pub fn default() -> Result<Self, String> {
        if let Some(provider) = get_provider_js().expect("No result from client") {
            return Ok(provider)
        } else {
            return Err("Error getting injected provider".into())
        }
    }

    // implement request (pub interface)
    pub async fn async_request(self, method: String, params: Vec<String> ) -> Result<JsValue, JsValue> {
        let args = RequestArguments{method, params};
        //log(format!("{:#?}", args).as_str());
        let ret = self.request(
            args,
            //&serde_wasm_bindgen::to_value(&args).unwrap(),
        );
        match ret.await {
            Ok(s)=> {
                let promise = Promise::resolve(&s.into());
                Ok(wasm_bindgen_futures::JsFuture::from(promise).await?)
            },
           Err(e) => Err(e)
        }
    }

    pub fn request_call(
        self,
        method: String,
        params: Vec<String>,
        ctx: Box<dyn Any>,
        callback: Box<dyn Fn(Result<JsValue, JsValue>, Box<dyn Any>) -> ()>
    ) -> () {
        spawn_local(async move {
            callback(self.async_request(method.clone(), params).await, ctx)
        });

    }



}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestArguments {
    method: String,
    params: Vec<String>,
}

#[wasm_bindgen]
impl RequestArguments {
    #[wasm_bindgen(constructor)]
    pub fn new(method: String, params: Vec<String>) -> Self {
        Self { method, params }
    }

    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.method.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn params(&self) -> Vec<String> {
        self.params.clone()
    }
}
/* 

/// Keep the provider and the event listeners attached to it together so we can remove them in the
/// `Drop` implementation. The logic can't go in Eip1193 because it's `Clone`, and cloning a JS
/// object just clones the reference.

#[derive(Debug)]
struct ProviderAndListeners {
    provider: Provider,
    listeners: BTreeMap<String, Vec<Closure<dyn FnMut(JsValue)>>>,
}

impl ProviderAndListeners {
    /// Listen for an event, and keep the listener closure for later cleanup.
    fn on(&mut self, name: &str, listener: Closure<dyn FnMut(JsValue)>) {
        self.provider.on(name, &listener);
        self.listeners
            .entry(name.to_owned())
            .or_insert(Vec::with_capacity(1))
            .push(listener);
    }
}

impl Drop for ProviderAndListeners {
    fn drop(&mut self) {
        for (event_name, listeners) in self.listeners.iter() {
            for listener in listeners.iter() {
                self.provider.removeListener(event_name, listener)
            }
        }
    }
}
 */