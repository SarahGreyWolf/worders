use game::{GameState, Player};

pub mod game;
pub mod packets;
pub mod thread_pool;

cfg_if::cfg_if! {
    if #[cfg(target_arch="wasm32")] {
        use wasm_bindgen::prelude::*;
        use web_sys::HtmlTableCellElement;
        use web_sys::WebSocket;
        use wasm_bindgen::JsCast;
        use util::*;
        use table::Table;
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_arch="wasm32")] {
        #[macro_use]
        mod util;
        mod table;
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn connect(hash: usize, id: usize) {
    let mut table = Table::new(15, 15, 2, None);
    let mut ws = WebSocket::new("ws://192.168.0.14:8080").unwrap();
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
}
    }
    console_log!("{:?}", game_state);
}
