use js_sys;
use serde_json;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod life;
use life::Cells;

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
    console::log_1(&JsValue::from_str("WASM module initialized"));

    Ok(())
}

#[wasm_bindgen]
pub struct Game {
    board: life::Board<life::Color>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Game {
    pub fn new(size: usize) -> Game {
        let mut board = life::Board::new(size, avg_color);

        board.randomize(size, 0.5, random_color);

        Game { board }
    }

    pub fn get_state(&self) -> String {
        let events: Vec<life::CellEvent<life::Color>> = self
            .board
            .cells()
            .into_iter()
            .map(|c| {
                life::CellEvent::Born(life::Cell {
                    coords: c.coords.clone(),
                    data: c.data.clone(),
                })
            })
            .collect();

        serde_json::to_string(&events).unwrap()
    }

    pub fn tick(&mut self) -> String {
        let events = self.board.tick();
        serde_json::to_string(&events).unwrap()
    }
}

fn random_color() -> life::Color {
    life::Color(rand::random(), rand::random(), rand::random())
}

fn avg_color(neighbours: Vec<life::CellView<life::Color>>) -> life::Color {
    neighbours
        .iter()
        .map(|c| c.data)
        .fold(life::Color(0., 0., 0.), |acc, c| {
            life::Color(
                acc.0 + c.0 / neighbours.len() as f32,
                acc.1 + c.1 / neighbours.len() as f32,
                acc.2 + c.2 / neighbours.len() as f32,
            )
        })
}
