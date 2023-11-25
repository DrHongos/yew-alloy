use ruint::{aliases::U256, Uint};
use yew::{function_component, html, Html, Properties};
/* 
TODO:
play more with presentation
linear-gradient
burn limit (and show)

*/
#[derive(Properties, PartialEq)]
pub struct Props {
    pub safe: U256,
    pub proposed: U256,
    pub fast: U256,
}

fn format_units(val: U256) -> usize {
    match val.checked_div(Uint::from(1000000000)) {
        Some(v) => v.try_into().unwrap(),
        None => 0
    }
}

#[function_component]
pub fn GasStation(props: &Props) -> Html {
    html! {
    <table 
        style= {format!("background: linear-gradient(rgba(1,1,{},0.4), rgba({}, 1, 1, 0.6))", format_units(props.safe), format_units(props.fast))}
        class={"gasStation"}
        >
        <tr>
            <th>{"Safe: "}</th>
            <td
                style={format!("font-size: {}px", format_units(props.safe))}
            >{format_units(props.safe)}</td>
        </tr>
        <tr>
            <th>{"Proposed: "}</th> 
            <td>{format_units(props.proposed)}</td>
        </tr>
        <tr>
            <th>{"Fast: "}</th>
            <td
                style={format!("font-size: {}px", format_units(props.fast))}
            >{format_units(props.fast)}</td>
        </tr>
    </table> 
    }
}



