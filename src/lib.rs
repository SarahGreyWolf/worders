pub mod game;
pub mod packets;
pub mod thread_pool;

cfg_if::cfg_if! {
    if #[cfg(target_arch="wasm32")] {
        use wasm_bindgen::prelude::*;
        use web_sys::{HtmlButtonElement};
        use web_sys::WebSocket;
        use wasm_bindgen::JsCast;
        use util::*;
        use table::Table;
        use packets::Packets;
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
    let mut table = Table::new(15, 15, 2, None, Some("board"), None);
    let mut hand = Table::new_no_height_aspect(7, 1, 2, None, Some("hand"), None);
    let ws = WebSocket::new("ws://192.168.0.14:8080").unwrap();
    let button = document().create_element("button").unwrap();
    button.set_class_name("confirm");
    let mut button_element = button.dyn_into::<HtmlButtonElement>().unwrap();
    button_element.set_inner_text("Confirm");
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    button_element.set_disabled(true);
    setup_closures(hash, id, ws, &mut table, &mut hand, &mut button_element);
    body()
        .append_child(&button_element)
        .expect("Failed to append button");
}

#[cfg(target_arch = "wasm32")]
fn setup_closures(
    hash: usize,
    id: usize,
    ws: WebSocket,
    table: &mut Table,
    hand: &mut Table,
    btn: &mut HtmlButtonElement,
) {
    use packets::{PacketFrom, PacketTo};

    let test = packets::PlayerState::new(
        0,
        0,
        "SarahGreyWolf".to_string(),
        vec!['S', 'A', 'R', 'A', 'H', ' ', ' '],
        4,
    );
    let _ws_clone = ws.clone();
    // On Connection Established
    let onopen = Closure::wrap(Box::new(move || {
        let mut send_buffer = vec![];
        let test_clone = test.clone();
        test_clone.encode(&mut send_buffer).unwrap();
        /*ws_clone
        .send_with_u8_array(&send_buffer.as_slice())
        .unwrap();*/
    }) as Box<dyn FnMut()>);
    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
    onopen.forget();

    // Cell clicked
    for cell in table.get_cells() {
        let ws_clone = ws.clone();
        let onclick = Box::new(move || {
            let mut character = ' ';
            if cell.element.inner_text() != "" {
                cell.element.set_inner_text("");
            } else {
                cell.element.set_inner_text("S");
                character = 'S';
            }
            let mut send_buffer = vec![];
            let placement =
                packets::Place::new(id as u16, character, cell.position[0], cell.position[1]);
            Packets::Place(placement).encode(&mut send_buffer).unwrap();
            assert!(send_buffer.len() > 0);
            ws_clone
                .send_with_u8_array(&send_buffer.as_slice())
                .unwrap();
        }) as Box<dyn FnMut()>;
        table
            .get_cell(cell.position[0], cell.position[1])
            .set_callback(onclick);
    }

    // Button Clicked
    let ws_clone = ws.clone();
    let onclick = Closure::wrap(Box::new(move || {
        let mut send_buffer = vec![];
        let confirm_packet = packets::Ack::new(id as u16, packets::AckState::Confirm);
        Packets::Ack(confirm_packet)
            .encode(&mut send_buffer)
            .unwrap();
        assert!(send_buffer.len() > 0);
        ws_clone
            .send_with_u8_array(&send_buffer.as_slice())
            .unwrap();
    }) as Box<dyn FnMut()>);
    btn.set_onclick(Some(onclick.as_ref().unchecked_ref()));
    onclick.forget();
}
