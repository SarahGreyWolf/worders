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
    setup_closures(hash, id, ws, &mut table);
}

#[cfg(target_arch = "wasm32")]
fn setup_closures(hash: usize, id: usize, ws: WebSocket, table: &mut Table) {
    use packets::{PacketFrom, PacketTo};

    let ws_clone = ws.clone();
    let mut id = 0;

    // On Connection Established
    let onopen = Closure::wrap(Box::new(move || {
    }) as Box<dyn FnMut()>);
    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
    onopen.forget();
    //Cell clicked
    for cell in table.get_cells() {
        let ws_clone = ws.clone();
        let onclick = Box::new(move || {
            let mut send_buffer = vec![];
            let placement = packets::Place::new(id, 'S', cell.position[0], cell.position[1]);
            placement.encode(&mut send_buffer);
            if send_buffer.len() > 0 {
                ws_clone
                    .send_with_u8_array(&send_buffer.as_slice())
                    .unwrap();
            }
        }) as Box<dyn FnMut()>;
        table
            .get_cell(cell.position[0], cell.position[1])
            .set_callback(onclick);
    }
}
