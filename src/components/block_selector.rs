use yew::prelude::*;
use alloy_rpc_types::BlockNumberOrTag;
use web_sys::HtmlInputElement;
use alloy_primitives::U64;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub from: bool,
    pub on_block_entry: Callback<(bool, BlockNumberOrTag)>,
}

#[function_component(BlockSelector)]
pub fn block_selector(props: &Props) -> Html {
    let block_number = use_state(|| U64::ZERO);             // set something different of 0
    let ops = use_state(|| BlockNumberOrTag::Latest);
    let dops = (*ops).clone();
    let onset_ops = {
        let c = ops.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let ns = match input.value().as_str() {
                "Finalized" => BlockNumberOrTag::Finalized,
                "Safe" => BlockNumberOrTag::Safe,
                "Pending" => BlockNumberOrTag::Pending,
                "Earliest" => BlockNumberOrTag::Earliest,
                "Number" => BlockNumberOrTag::Number(U64::ZERO),
                &_ => BlockNumberOrTag::Latest,
            };
            c.set(ns);
        })
    };
    
    let set_block_number = {
        let c = block_number.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let ns = input.value().parse::<u64>().expect("Invalid value");
            c.set(U64::from(ns));
        })
    };
    
    let ret = {
        let pprops = props.on_block_entry.clone();    
        let from = props.from.clone();
        Callback::from(move |_| {
            let res = if dops.is_number() {
                BlockNumberOrTag::Number(*block_number)
            } else {
                *ops
            };
            pprops.emit((from, res))
        })
    };

    html! (
        <div class={"block_selector"}>
            <select onchange={onset_ops}>
                <option value="Finalized">{"Finalized"}</option>
                <option value="Safe">{"Safe"}</option>
                <option value="Earliest">{"Earliest"}</option>
                <option value="Pending">{"Pending"}</option>
                <option value="Number">{"Number"}</option>
                <option value="Latest">{"Latest"}</option>
            </select>
            if dops.is_number() {
                <input onchange={set_block_number} class={"block_number_input"} type="number" />
            }
            <button onclick={ret}>{"Set"}</button>            
        </div>
    )
}