use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CellEvent {
    Born(Cell),
    Died(Cell),
}

#[derive(Debug)]
pub struct Board {
    cell_map: HashMap<Coord, Color>,
    size: usize,
}

pub trait Cells {
    fn get(&self, coord: &Coord) -> Option<CellView>;
    fn insert(&mut self, coord: Coord, color: Color) -> bool;
    fn remove(&mut self, coord: &Coord);
    fn get_neighbours(&self, coord: &Coord) -> Vec<CellView>;
    fn all_dead_neighbours(&self) -> Vec<Coord>;
    fn cells(&self) -> Vec<CellView>;
}

static NEIGHBOURS_DIFFS: &'static [(i32, i32)] = &[
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

impl Cells for Board {
    fn get(&self, coords: &Coord) -> Option<CellView> {
        self.cell_map
            .get_key_value(coords)
            .map(|(coords, color)| CellView { coords, color })
    }

    fn remove(&mut self, coords: &Coord) {
        self.cell_map.remove(coords);
    }

    fn insert(&mut self, coords: Coord, color: Color) -> bool {
        if coords.0 < 0
            || coords.1 < 0
            || coords.0 >= self.size as i32
            || coords.1 >= self.size as i32
        {
            false
        } else {
            self.cell_map.insert(coords, color);
            true
        }
    }

    fn get_neighbours(&self, coord: &Coord) -> Vec<CellView> {
        NEIGHBOURS_DIFFS
            .iter()
            .filter_map(|(dx, dy)| {
                let (coords, color) = self
                    .cell_map
                    .get_key_value(&Coord(coord.0 + dx, coord.1 + dy))?;

                Some(CellView { coords, color })
            })
            .collect()
    }

    fn all_dead_neighbours(&self) -> Vec<Coord> {
        let possible_empty: HashSet<Coord> = self
            .cell_map
            .keys()
            .flat_map(|coords| {
                NEIGHBOURS_DIFFS
                    .iter()
                    .map(move |(dx, dy)| Coord(coords.0 + dx, coords.1 + dy))
            })
            .collect();

        possible_empty
            .into_iter()
            .filter_map(|coords| match self.cell_map.get(&coords) {
                Some(_) => None,
                None => Some(coords),
            })
            .collect()
    }

    fn cells(&self) -> Vec<CellView> {
        self.cell_map
            .iter()
            .map(|(coord, color)| CellView {
                coords: coord,
                color,
            })
            .collect::<Vec<CellView>>()
    }
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            size,
            cell_map: HashMap::new(),
        }
    }

    pub fn random(size: usize, alive_chance: f32) -> Board {
        let mut board = Board::new(size);

        for i in 0..size {
            for j in 0..size {
                if rand::random::<f32>() <= alive_chance {
                    board.insert(
                        Coord(i as i32, j as i32),
                        Color(rand::random(), rand::random(), rand::random()),
                    );
                }
            }
        }

        board
    }

    pub fn tick(&mut self) -> Vec<CellEvent> {
        let mut updates: Vec<CellEvent> = vec![];

        for CellView { coords, color } in self.cells().iter() {
            let neighbours = self.get_neighbours(coords);
            if neighbours.len() < 2 || neighbours.len() > 3 {
                updates.push(CellEvent::Died(Cell {
                    coords: (*coords).clone(),
                    color: (*color).clone(),
                }))
            }
        }

        for coords in self.all_dead_neighbours() {
            let alive_neighbours = self.get_neighbours(&coords);
            if alive_neighbours.len() == 3 {
                let color =
                    alive_neighbours
                    .iter()
                    .map(|c| c.color)
                    .fold(Color(0., 0., 0.), |acc, c| {
                        Color(
                            acc.0 + c.0 / alive_neighbours.len() as f32,
                            acc.1 + c.1 / alive_neighbours.len() as f32,
                            acc.2 + c.2 / alive_neighbours.len() as f32,
                        )
                    });

                updates.push(CellEvent::Born(Cell { coords, color }))
            }
        }

        for update in &updates {
            match update {
                CellEvent::Born(cell) => {
                    self.insert(cell.coords.clone(), cell.color.clone());
                }
                CellEvent::Died(cell) => self.remove(&cell.coords),
            };
        }

        updates
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coord(pub i32, pub i32);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Color(pub f32, pub f32, pub f32);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Cell {
    pub coords: Coord,
    pub color: Color,
}

#[derive(Debug, PartialEq)]
pub struct CellView<'a> {
    pub coords: &'a Coord,
    pub color: &'a Color,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn red() -> Color {
        Color(1., 0., 0.)
    }

    #[test]
    fn empty_stays_empty() {
        let mut board = Board::new(100);

        assert_eq!(board.tick(), vec![])
    }

    #[test]
    fn single_cell_dies() {
        let mut board = Board::new(100);

        board.insert(Coord(0, 0), red());

        board.tick();

        assert_eq!(board.cells(), vec![])
    }

    #[test]
    fn cell_with_one_neighbour_dies() {
        let mut board = Board::new(100);

        board.insert(Coord(0, 0), red());
        board.insert(Coord(1, 1), red());
        board.insert(Coord(1, 2), red());

        board.tick();

        assert_eq!(board.cell_map.get(&Coord(0, 0,)), None)
    }

    #[test]
    fn crowded_cell_dies() {
        let cells = vec![
            Coord(0, 0),
            Coord(0, 1),
            Coord(0, 2),
            Coord(1, 1),
            Coord(2, 0),
            Coord(2, 1),
        ];

        let mut board = Board::new(100);

        cells.into_iter().for_each(|c| {
            board.insert(c, red());
        });

        board.tick();

        assert_eq!(board.cell_map.get(&Coord(1, 1,)), None)
    }

    #[test]
    fn cell_comes_alive() {
        let cells = vec![Coord(0, 0), Coord(0, 1), Coord(0, 2)];

        let mut board = Board::new(100);

        cells.into_iter().for_each(|c| {
            board.insert(c, red());
        });

        board.tick();

        board.cell_map.get(&Coord(1, 1)).expect("Should come alive");
        board
            .cell_map
            .get(&Coord(-1, 1))
            .expect("Should come alive");
    }
}

#[cfg(test)]
mod snapshots {
    use super::*;
    use insta::*;

    fn red() -> Color {
        Color(1., 0., 0.)
    }

    fn display_board(board: &Board) -> String {
        let mut display = String::new();

        for i in 0..board.size {
            for j in 0..board.size {
                display.push(match board.get(&Coord(i as i32, j as i32)) {
                    Some(_) => 'x',
                    None => ' ',
                })
            }
            display.push('\n');
        }

        display
    }

    fn test_board_tick(board: &mut Board) {
        let before = display_board(board);

        let events = board
            .tick()
            .iter()
            .map(|e| format!("{:?}", e))
            .collect::<Vec<_>>()
            .join("\\n");

        let after = display_board(&board);

        assert_snapshot!(format!(
            "Before:\n{}\nAfter:\n{}\nEvents: {}",
            before, after, events
        ));
    }

    #[test]
    fn test_horizontal_line() {
        let mut board = Board::new(3);

        board.insert(Coord(1, 0), red());
        board.insert(Coord(1, 1), red());
        board.insert(Coord(1, 2), red());

        test_board_tick(&mut board);
    }

    #[test]
    fn test_vertical_line() {
        let mut board = Board::new(3);

        board.insert(Coord(0, 1), red());
        board.insert(Coord(1, 1), red());
        board.insert(Coord(2, 1), red());

        test_board_tick(&mut board);
    }

    #[test]
    fn test_neighbours() {
        let mut board = Board::new(3);

        board.insert(Coord(0, 0), red());
        board.insert(Coord(1, 0), red());
        board.insert(Coord(2, 0), red());

        board.insert(Coord(0, 1), red());
        board.insert(Coord(1, 1), red());
        board.insert(Coord(2, 1), red());

        board.insert(Coord(0, 2), red());
        board.insert(Coord(1, 2), red());
        board.insert(Coord(2, 2), red());

        assert_debug_snapshot!(board
            .get_neighbours(&Coord(1, 1))
            .iter()
            .map(|c| c.coords)
            .collect::<Vec<_>>())
    }
}
