use yew::prelude::*;
use alloy_primitives::Address;
use web_sys::HtmlInputElement;
use crate::contexts::ethereum::UseEthereum;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_add: Callback<Address>,
    pub show_me: bool,
    pub placeholder: String,
}

#[function_component(AddressInput)]
pub fn address_input(props: &Props) -> Html {
    let error_msg = use_state(|| None as Option<String>);
    
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );
    let main_account = ethereum.account();
    
    let on_change_address = {
        let propsr = props.on_add.clone();
        let err = error_msg.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            match input.value().parse::<Address>() {
                Ok(a) => {
                    err.set(None);
                    propsr.emit(a);
                },
                Err(e) => err.set(Some(e.to_string()))
            }
        })
    };

    let set_my_address = {
        let main_account = main_account.clone();
        let propsr = props.on_add.clone();
        Callback::from(move |_| {
            //let ma = main_account.clone();
            propsr.emit(main_account);
        })
    };

    html! {
        <div>
            <div class="address_input">
                <input 
                    onchange={on_change_address} 
                    id={"address_input"} 
                    class={"address_input"} 
                    placeholder={props.placeholder.clone()}
                    type="text" 
                />
                if props.show_me {
                    <button onclick={set_my_address} class={"button"}>{"me"}</button>
                }
            </div>
            if let Some(error) = (*error_msg).clone() {
                <hr />
                <small>{error}</small>
            }
        </div>
    }
}