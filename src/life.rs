use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellEvent<D> {
    Born(Cell<D>),
    Died(Cell<D>),
}

// #[derive(Debug)]
pub struct Board<D: Clone = ()> {
    cell_map: HashMap<Coord, D>,
    size: usize,
    new_cell_data: fn(Vec<CellView<D>>) -> D,
}

pub trait Cells<D> {
    fn get(&self, coord: &Coord) -> Option<CellView<D>>;
    fn insert(&mut self, coord: Coord, data: D) -> bool;
    fn remove(&mut self, coord: &Coord);
    fn get_neighbours(&self, coord: &Coord) -> Vec<CellView<D>>;
    fn all_dead_neighbours(&self) -> Vec<Coord>;
    fn cells(&self) -> Vec<CellView<D>>;
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

impl<D: Clone> Cells<D> for Board<D> {
    fn get(&self, coords: &Coord) -> Option<CellView<D>> {
        self.cell_map
            .get_key_value(coords)
            .map(|(coords, data)| CellView { coords, data })
    }

    fn remove(&mut self, coords: &Coord) {
        self.cell_map.remove(coords);
    }

    fn insert(&mut self, coords: Coord, data: D) -> bool {
        if coords.0 < 0
            || coords.1 < 0
            || coords.0 >= self.size as i32
            || coords.1 >= self.size as i32
        {
            false
        } else {
            self.cell_map.insert(coords, data);
            true
        }
    }

    fn get_neighbours(&self, coord: &Coord) -> Vec<CellView<D>> {
        NEIGHBOURS_DIFFS
            .iter()
            .filter_map(|(dx, dy)| {
                let (coords, data) = self
                    .cell_map
                    .get_key_value(&Coord(coord.0 + dx, coord.1 + dy))?;

                Some(CellView { coords, data })
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

    fn cells(&self) -> Vec<CellView<D>> {
        self.cell_map
            .iter()
            .map(|(coords, data)| CellView { coords, data })
            .collect::<Vec<CellView<_>>>()
    }
}

impl<D: Clone> Board<D> {
    pub fn new(size: usize, new_cell_data: fn(Vec<CellView<D>>) -> D) -> Board<D> {
        Board {
            size,
            cell_map: HashMap::new(),
            new_cell_data,
        }
    }

    pub fn randomize(&mut self, size: usize, alive_chance: f32, get_default_data: fn() -> D) {
        for i in 0..size {
            for j in 0..size {
                if rand::random::<f32>() <= alive_chance {
                    self.insert(Coord(i as i32, j as i32), get_default_data());
                }
            }
        }
    }

    pub fn tick(&mut self) -> Vec<CellEvent<D>> {
        let mut updates: Vec<CellEvent<D>> = vec![];

        for CellView { coords, data } in self.cells().iter() {
            let neighbours = self.get_neighbours(coords);
            if neighbours.len() < 2 || neighbours.len() > 3 {
                updates.push(CellEvent::Died(Cell {
                    coords: (*coords).clone(),
                    data: (*data).clone(),
                }))
            }
        }

        for coords in self.all_dead_neighbours() {
            let alive_neighbours = self.get_neighbours(&coords);
            if alive_neighbours.len() == 3 {
                let data = (self.new_cell_data)(alive_neighbours);

                updates.push(CellEvent::Born(Cell { coords, data }))
            }
        }

        for update in &updates {
            match update {
                CellEvent::Born(cell) => {
                    self.insert(cell.coords.clone(), cell.data.clone());
                }
                CellEvent::Died(cell) => self.remove(&cell.coords),
            };
        }

        updates
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord(pub i32, pub i32);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Color(pub f32, pub f32, pub f32);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cell<D> {
    pub coords: Coord,
    pub data: D,
}

#[derive(Debug, PartialEq)]
pub struct CellView<'a, D> {
    pub coords: &'a Coord,
    pub data: &'a D,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_board(size: usize) -> Board {
        Board::new(size, |_cells| ())
    }

    #[test]
    fn empty_stays_empty() {
        let mut board = test_board(100);

        assert_eq!(board.tick(), vec![])
    }

    #[test]
    fn single_cell_dies() {
        let mut board = test_board(100);

        board.insert(Coord(0, 0), ());

        board.tick();

        assert_eq!(board.cells(), vec![])
    }

    #[test]
    fn cell_with_one_neighbour_dies() {
        let mut board = test_board(100);

        board.insert(Coord(0, 0), ());
        board.insert(Coord(1, 1), ());
        board.insert(Coord(1, 2), ());

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

        let mut board = test_board(100);

        cells.into_iter().for_each(|c| {
            board.insert(c, ());
        });

        board.tick();

        assert_eq!(board.cell_map.get(&Coord(1, 1,)), None)
    }

    #[test]
    fn cell_comes_alive() {
        let cells = vec![Coord(0, 0), Coord(0, 1), Coord(0, 2)];

        let mut board = test_board(100);

        cells.into_iter().for_each(|c| {
            board.insert(c, ());
        });

        board.tick();

        board.cell_map.get(&Coord(1, 1)).expect("Should come alive");
    }
}

#[cfg(test)]
mod snapshots {
    use super::*;
    use insta::*;

    fn test_board(size: usize) -> Board {
        Board::new(size, |_cells| ())
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

        let mut events = board.tick();

        events.sort();

        let events = events
            .iter()
            .map(|e| format!("{:?}", e))
            .collect::<Vec<_>>()
            .join("\n");

        let after = display_board(&board);

        assert_snapshot!(format!(
            "Before:\n{}\nAfter:\n{}\nEvents: {}",
            before, after, events
        ));
    }

    #[test]
    fn test_horizontal_line() {
        let mut board = test_board(3);

        board.insert(Coord(1, 0), ());
        board.insert(Coord(1, 1), ());
        board.insert(Coord(1, 2), ());

        test_board_tick(&mut board);
    }

    #[test]
    fn test_vertical_line() {
        let mut board = test_board(3);

        board.insert(Coord(0, 1), ());
        board.insert(Coord(1, 1), ());
        board.insert(Coord(2, 1), ());

        test_board_tick(&mut board);
    }

    #[test]
    fn test_neighbours() {
        let mut board = test_board(3);

        board.insert(Coord(0, 0), ());
        board.insert(Coord(1, 0), ());
        board.insert(Coord(2, 0), ());

        board.insert(Coord(0, 1), ());
        board.insert(Coord(1, 1), ());
        board.insert(Coord(2, 1), ());

        board.insert(Coord(0, 2), ());
        board.insert(Coord(1, 2), ());
        board.insert(Coord(2, 2), ());

        assert_debug_snapshot!(board
            .get_neighbours(&Coord(1, 1))
            .iter()
            .map(|c| c.coords)
            .collect::<Vec<_>>())
    }
}
