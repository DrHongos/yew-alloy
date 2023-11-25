use yew::prelude::*;
use crate::components::track::Track;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Ahoy Alloy!" }</h1>
            <a href="https://github.com/alloy-rs">{"github"}</a>
            /* <HelloWorld 
                is_loading=true
            /> */
            <Track />
        </main>
    }
}
