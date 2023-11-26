//use std::rc::Rc;
use yew::prelude::*;
use alloy_chains::Chain;
/* 
    Error: Cannot render children (IntoProps not implemented!?)

*/
/* 
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MetamaskMsg {
    pub inner: String,
}

impl Reducible for MetamaskMsg {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        MetamaskMsg { inner: action }.into()
    }
}

pub type MetamaskMsgContext = UseReducerHandle<MetamaskMsg>;
 */

#[derive(Clone, Debug, PartialEq)]
pub struct MMContext {
    account: Option<String>,
    chain: Option<Chain>,
}

 #[derive(Properties, Debug, PartialEq)]
pub struct MetamaskProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn MetamaskProvider(props: &MetamaskProviderProps) -> Html {
/*     let msg = use_reducer(|| MetamaskMsg {
        inner: "No message yet.".to_string(),
    });
 */
    let ctx = use_state(|| MMContext {
        account: None,
        chain: None,
    });
    html! {
        <ContextProvider<MMContext> context={(*ctx).clone()}/* <MetamaskMsgContext> context={msg} */>
            {props.children.clone()}
        </ContextProvider<MMContext>/* <MetamaskMsgContext> */>
    }
}

