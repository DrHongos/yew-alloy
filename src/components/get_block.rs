use yew::prelude::*;
use crate::contexts::ethereum::UseEthereum;
use alloy_rpc_types::Block;
use std::ops::BitXorAssign;
use crate::components::{
    get_block_by_number::GetBlockByNumber,
    get_block_by_hash::GetBlockByHash,
};
use web_sys::HtmlInputElement;

// makes returned Block -> displays

#[function_component(GetBlock)]
pub fn get_block() -> Html {
    let open = use_state(|| false);
    let method = use_state(|| "number".to_string());
    let block = use_state(|| None as Option<Block>);        // received and to be displayed
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let toggle_comp = {
        let o = open.clone();
        let rv = (*o).clone();
        Callback::from(move |_e: MouseEvent| {
            let mut rvv = rv.clone();
            let _ = rvv.bitxor_assign(true);
            o.set(rvv);
        })
    };
    let button_label = match (*open).clone() {
        true => "Cancel",
        false => "Blocks",
    };
    let onset_method = {
        let m = method.clone();
        Callback::from(move |e: Event| {
            let v: HtmlInputElement = e.target_unchecked_into();
            m.set(v.value());
        })
    };
    let onset_block: Callback<Block> = {
        let b = block.clone();
        Callback::from(move |inp: Block| {
            crate::helpers::log(format!("block: {:#?}", inp).as_str());
            b.set(Some(inp));
        })
    };
    let tools = {match (*method).as_str() {
        "number" => html! {
            <GetBlockByNumber
                on_block={onset_block}
            />
        },
        "hash" => html! {
            <GetBlockByHash
                on_block={onset_block}
            />
        },
        _ => html! {<p>{"Something wrong has occured"}</p>},
    }};

    html!{
        <div class={"getCode"}>
            if ethereum.is_connected() {
                <button onclick={toggle_comp} class={"button"}>
                    {button_label}
                </button>
                if (*open).clone() {
                    <select onchange={onset_method}>
                        <option value="hash">{"Hash"}</option>
                        <option value="number">{"Number"}</option>
                    </select>
                    {tools}
                    // display Block
                }
            }
        </div>
    }
}