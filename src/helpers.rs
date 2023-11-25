// HELPERS
use ruint::{
    Uint,
    aliases::U256
};

pub fn format_gas(val: U256) -> usize {
    match val.checked_div(Uint::from(1000000000)) {
        Some(v) => v.try_into().unwrap(),
        None => 0
    }
}