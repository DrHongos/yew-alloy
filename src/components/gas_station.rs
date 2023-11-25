use ruint::aliases::U256;
use yew::{function_component, html, Html, Properties};
use crate::helpers::format_gas;
/* 
TODO:
make a sole component, nice to the eye!

*/
#[derive(Properties, PartialEq)]
pub struct Props {
    pub safe: U256,
    pub proposed: U256,
    pub fast: U256,
}


#[function_component]
pub fn GasStation(props: &Props) -> Html {
    html! {
    <table 
        style= {format!("background: linear-gradient(rgba(1,1,{},0.4), rgba({}, 1, 1, 0.6))", format_gas(props.safe), format_gas(props.fast))}
        class={"gasStation"}
        >
        if format_gas(props.safe) >= 22 {
            <i class="burn" />
        }
        <tr>
            <th>{"Safe: "}</th>
            <td
                style={format!("font-size: {}px", format_gas(props.safe))}
            >{format_gas(props.safe)}</td>
        </tr>
        <tr>
            <th>{"Proposed: "}</th> 
            <td>{format_gas(props.proposed)}</td>
        </tr>
        <tr>
            <th>{"Fast: "}</th>
            <td
                style={format!("font-size: {}px", format_gas(props.fast))}
            >{format_gas(props.fast)}</td>
        </tr>
    </table> 
    }
}



