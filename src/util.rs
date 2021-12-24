use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("No global window exists")
}

pub fn document() -> web_sys::Document {
    window().document().expect("No Document for window exists")
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("No body for document exists")
}
