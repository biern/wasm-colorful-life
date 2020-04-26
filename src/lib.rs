use js_sys;
use serde_json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
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
    canvas: web_sys::HtmlCanvasElement,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Game {
    pub fn new(size: usize, canvas: web_sys::HtmlCanvasElement) -> Game {
        let mut board = life::Board::new(size, avg_color_with_mutation);

        board.randomize(size, 0.5, random_color);

        Game { board, canvas }
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

    pub fn tick(&mut self) {
        self.board.tick();
    }

    pub fn draw(&mut self, cell_size: f64) {
        let context = self
            .canvas
            .get_context(&"2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        for cell in self.board.cells() {
            context.save();
            context.set_line_width(1.);
            context
                .translate(
                    cell.coords.0 as f64 * cell_size,
                    cell.coords.1 as f64 * cell_size,
                )
                .unwrap();
            context.set_fill_style(
                &format!(
                    "rgb({}, {}, {})",
                    (cell.data.0 * 255.).floor(),
                    (cell.data.1 * 255.).floor(),
                    (cell.data.2 * 255.).floor(),
                )
                .into(),
            );
            context.fill_rect(0., 0., cell_size, cell_size);
            context.restore();
        }
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

fn avg_color_with_mutation(neighbours: Vec<life::CellView<life::Color>>) -> life::Color {
    let mut avg = neighbours
        .iter()
        .map(|c| c.data)
        .fold(life::Color(0., 0., 0.), |acc, c| {
            life::Color(
                acc.0 + c.0 / neighbours.len() as f32,
                acc.1 + c.1 / neighbours.len() as f32,
                acc.2 + c.2 / neighbours.len() as f32,
            )
        });

    let mutation_chance = 0.01;

    if rand::random::<f32>() <= mutation_chance {
        avg.0 = rand::random();
    }

    if rand::random::<f32>() <= mutation_chance {
        avg.1 = rand::random();
    }

    if rand::random::<f32>() <= mutation_chance {
        avg.2 = rand::random();
    }

    avg
}
