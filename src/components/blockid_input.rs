use yew::prelude::*;
use alloy_rpc_types::{BlockId, BlockNumberOrTag};
use web_sys::HtmlInputElement;
use std::str::FromStr;
use alloy_primitives::{FixedBytes, B256};
use crate::components::block_selector::BlockSelector;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_block_entry: Callback<BlockId>,
}

#[function_component(BlockIdInput)]
pub fn blockid_input(props: &Props) -> Html {
    let block_selected = use_state(|| "".to_string());

    let onset_ops = {
        let b = block_selected.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            b.set(input.value());
    })};
    
    let set_block_number: Callback<(bool, BlockNumberOrTag)> = {
        let pprops = props.on_block_entry.clone();    
        Callback::from(move |b| {
            let (_f, b) = b;
            pprops.emit(BlockId::Number(b));            
        })
    };

    let set_block_hash = {
        let pprops = props.on_block_entry.clone();
        Callback::from(move |e: Event| {
            let v: HtmlInputElement = e.target_unchecked_into();
            let p = FixedBytes::from_str(&v.value());
            if let Ok(p) = p {
                pprops.emit(
                    BlockId::Hash(
                        B256::from(p).into()
                    )
                )
            }
        })
    };
    let inputs = match (*block_selected).as_str() {
        "hash" => {
            html!{
                <div class={"B256_input"}>
                    <label for={"b256"}>{"blockhash"}</label>
                    <input onchange={set_block_hash} id={"b256"} class={"address_input"} type="text" />
                </div>
            }
        },
        "number" => {
            html!{
                <BlockSelector
                    from={true}
                    on_block_entry={set_block_number}
                />
            }
        },
        _ => html!{}
    };

    html! (
        <div class={"block_selector"}>
            <select onchange={onset_ops}>
                <option value="hash">{"Hash"}</option>
                <option value="number">{"Number or Tag"}</option>
            </select>
            {inputs}
        </div>
    )
}