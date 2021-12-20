use game::{GameState, Player};

mod game;

cfg_if::cfg_if! {
    if #[cfg(target_arch="wasm32")] {
        use wasm_bindgen::prelude::*;
        use web_sys::HtmlTableCellElement;
        use util::*;
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
    table::Table::new(15, 15, 2, None);
    let mut game_state = GameState::new();
    if !game_state.add_player(Player::new("SarahGreyWolf")) {
        console_log!("Something went wrong!");
    }
    console_log!("{:?}", game_state);
}
