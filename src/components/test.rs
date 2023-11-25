use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
}

#[function_component]
pub fn HelloWorld(props: &Props) -> Html {
    if props.is_loading.clone() {
        html! { "Loading" }
    } else {
        html! { "Ahoy Alloy Again!" }
    }
}
/* 
// Then use like this with default
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// Or no override the default
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld is_loading={true} />}
} 
*/