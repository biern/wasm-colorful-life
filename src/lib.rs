use js_sys;
use serde_json;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod life;
use life::{Cells, Color, Coord};
mod sim;

use sim::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
pub struct Game {
    board: life::Board,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Game {
    pub fn new(size: usize) -> Game {
        let board = life::Board::random(size, 0.5);

        Game { board }
    }

    pub fn tick(&mut self) -> String {
        console::log_1(&JsValue::from_str("game tick"));
        let events = self.board.tick();
        serde_json::to_string(&events).unwrap()
    }

    pub fn callback(&self, f: js_sys::Function) {
        console::log_1(&JsValue::from_str("before"));
        f.call0(&JsValue::NULL).unwrap();
    }
}
