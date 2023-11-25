use yew::{function_component, Html, html, Properties};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/jscripts/multi_lineal_chart.js")]
extern "C" {
    #[wasm_bindgen(js_name = "LineChart")]
    #[wasm_bindgen(catch)]
    pub fn LineChart(data: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Properties, PartialEq)]
pub struct LinealChartProps {
    pub list_to_display: JsValue,//Vec<Tick>,
}

#[function_component(LinealChart)]
pub fn render_lineal_chart(props: &LinealChartProps) -> Html {
    match LineChart(props.list_to_display.clone()) {
        Ok(_) => {                        
            ()
        },
        Err(_err) => {
//            log::error!("Plotting failed! {:?}", err);
            ()
        }
    };

    html! {
        <>
            <div 
                id="chart"
                style={"
                background-color: white; 
                color: black; 
                padding: 20px;
                border-radius: 10px
                "}
            ></div>
        </>
    }
}