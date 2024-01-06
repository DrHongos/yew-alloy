use yew::prelude::*;
use alloy_primitives::{Address, FixedBytes, B256};
use alloy_rpc_types::{BlockNumberOrTag, Filter};
use std::str::FromStr;
use crate::helpers::log;
use crate::contexts::ethereum::UseEthereum;
use web_sys::HtmlInputElement;
use crate::components::{
    block_selector::BlockSelector,
    address_input::AddressInput,
};
/* 
TODO:
    - replace address inputs for the component (in here and others)

    - finish the filter creation (range vs blockhash)
    - add topics
    - handle error cases

*/

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_filter: Callback<Filter>,
}

#[function_component(FilterCreator)]
pub fn filter_creator(props: &Props) -> Html {
    let addresses = use_state(|| Vec::<Address>::new());
    
    let range_type = use_state(|| "one".to_string());
    let block_hash = use_state(|| B256::from(FixedBytes::ZERO));
    let from_block = use_state(|| BlockNumberOrTag::Latest);
    let to_block = use_state(|| BlockNumberOrTag::Latest);

    let events = use_state(|| Vec::new());
    
    let topic_1 = use_state(|| String::new());  // topics are B256 
    let topic_2 = use_state(|| String::new());// topics are B256 
    let topic_3 = use_state(|| String::new());// topics are B256 

    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );

    let set_range_type = {
        let r = range_type.clone();
        Callback::from(move |e: Event| {
            let v: HtmlInputElement = e.target_unchecked_into();
            r.set(v.value());
        })
    };
    let set_block_hash = {
        let bh = block_hash.clone();
        Callback::from(move |e: Event| {
            let v: HtmlInputElement = e.target_unchecked_into();
            let p = FixedBytes::from_str(&v.value());
            if let Ok(p) = p {
                bh.set(B256::from(p))
            } else {
                bh.set(B256::from(FixedBytes::ZERO))
            }
        })
    };

    let on_block_entry: Callback<(bool, BlockNumberOrTag)> = {
        let b = from_block.clone();
        let s = to_block.clone();
        Callback::from(move |inp| {
            let (from, block) = inp;
            //            log(format!("Received {:#?}", block).as_str());
            if from {
                b.set(block);
            } else {
                s.set(block);
            }
        })
    };

    let on_add_address: Callback<Address> = {
        let a = addresses.clone();
        Callback::from(move |addr| {
            if a.contains(&addr) {
                log("Loaded already");
            } else {
                let mut c = (*a).clone();
                let _ = c.push(addr);
                a.set(c);
            }
        })
    };

    let set_topic_1 = {
        let t = topic_1.clone();
        Callback::from(move |e: Event| {
            let h: HtmlInputElement = e.target_unchecked_into();
            t.set(h.value());
        })
    };
    let set_topic_2 = {
        let t = topic_2.clone();
        Callback::from(move |e: Event| {
            let h: HtmlInputElement = e.target_unchecked_into();
            t.set(h.value());
        })
    }; 
    let set_topic_3 = {
        let t = topic_3.clone();
        Callback::from(move |e: Event| {
            let h: HtmlInputElement = e.target_unchecked_into();
            t.set(h.value());
        })
    };

    let add_event = {
        //let filter = filter.borrow_mut();
        let ev = events.clone();
        Callback::from(move |e: Event| {
            //let f = filter.borrow_mut();
            let h: HtmlInputElement = e.target_unchecked_into();
            let v = h.value();
            //log(format!("is {}", v).as_str());
            if ev.contains(&v) {
                log("Loaded already");
            } else {
                let mut c = (*ev).clone();
                let _ = c.push(v);
                ev.set(c);
            }
            // check is ok?
            //filter.event(&v);
        })            
    };

    let create_filter = {
        let events = events.clone();
        let addresses = addresses.clone();
        let from_block = from_block.clone();
        let to_block = to_block.clone();
        let propse = props.on_filter.clone();
        Callback::from(move |_| {
            let filter = Filter::new()
                .address((*addresses).clone())
                .events((*events).clone())
                .from_block((*from_block).clone())      // range
                .to_block((*to_block).clone());
                //.at_block_hash((*block_hash).clone())     one
               // .topic1((*topic_1).clone())
               // .topic2((*topic_2).clone())
               // .topic3((*topic_3).clone());
            // if range || hash

            // if events
            //let _ = filter.events((*events).clone());
            // if indexed topics add them
            
            // then call parent with result
            log(format!("{:#?}", filter).as_str());
            propse.emit(filter.clone());
        })
    };

    // UI
    let addresses_list = (*addresses)
        .clone()
        .into_iter()
        .map(|a| 
            html!(
                <div class="name_remove">
                    <p>{a.to_string()}</p>
                    <button onclick={
                        let addresses = addresses.clone();
                        Callback::from(move |_| {
                            log(format!("{}",a).as_str());
                            let index = addresses.clone().iter().position(|x| *x == a).unwrap();
                            let mut c = (*addresses).clone();
                            c.remove(index);
                            addresses.set(c.to_vec());
                    })}>{"X"}</button>
                </div>
            )
        )
        .collect::<Html>();

    let events_list = (*events)
        .clone()
        .into_iter()
        .map(|e|
            html!(
                <div class="name_remove">
                    <p>{e.to_string()}</p>
                    <button onclick={
                        let events = events.clone();
                        Callback::from(move |_| {
                            log(format!("{}",e).as_str());
                            let index = events.clone().iter().position(|x| *x == e).unwrap();
                            let mut c = (*events).clone();
                            c.remove(index);
                            events.set(c.to_vec());
                    })}>{"X"}</button>
                </div>
            )
        )
        .collect::<Html>();


    html!{
        <div class={"filter"}>
            if ethereum.is_connected() {
                <small>{"Add addresses"}</small>
                <AddressInput 
                    on_add={on_add_address}
                    show_me={true}
                />
                // list addresses to listen to (and remove button)
                {addresses_list}
                <hr />
                <small>{"Define "}</small>
                // get blocks checked
                
                <select onchange={set_range_type}>
                    <option value="many">{"Range"}</option>
                    <option value="one">{"Block"}</option>
                </select>
                if *range_type == "one" {
                    <div class={"B256_input"}>
                        <label>{"blockhash"}</label>
                        <input onchange={set_block_hash} class={"address_input"} type="text" />
                        if (*block_hash) != B256::from(FixedBytes::ZERO) {
                            <i class="valid" />
                        } else {
                            <i class="invalid" />
                        }
                    </div>
                } else {
                    <div class={"range_input"}>
                        <label>{"from"}</label>
                        <BlockSelector 
                            from={true}
                            on_block_entry={on_block_entry.clone()}
                        />
                        <label>{"to"}</label>
                        <BlockSelector 
                            from={false}
                            on_block_entry={on_block_entry}
                        />
                    </div>
                }
                
                <hr />
                <small>{"Events"}</small>
                // get events listened
                <input onchange={add_event} placeholder={"event signature"} class={"address_input"} type="text" /> 
                {events_list}
                <hr />
                // topics
                <small>{"Indexed topics"}</small>
                <input onchange={set_topic_1} placeholder={"topic 1"} class={"address_input"} type="text" /> 
                <input onchange={set_topic_2} placeholder={"topic 2"} class={"address_input"} type="text" />
                <input onchange={set_topic_3} placeholder={"topic 3"} class={"address_input"} type="text" />
                <hr />
                <button onclick={create_filter} class={"button"}>{"Create filter"}</button>
            }
        </div>
    }
}
