use wasm_bindgen::prelude::*;
use util::*;

#[macro_use]
mod util;
mod table;



#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    table::Table::new(10, 10, 2, None);
    console_log!("Fuuck?");
}