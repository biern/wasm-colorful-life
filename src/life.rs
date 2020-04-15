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
    fn insert(&mut self, coord: Coord, color: Color);
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
    (-1, 1),
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

    fn insert(&mut self, coords: Coord, color: Color) {
        self.cell_map.insert(coords, color);
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
                updates.push(CellEvent::Born(Cell {
                    coords,
                    color: Color(1.0, 1.0, 1.0),
                }))
            }
        }

        for update in &updates {
            match update {
                CellEvent::Born(cell) => self.insert(cell.coords.clone(), cell.color.clone()),
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
    coords: Coord,
    color: Color,
}

#[derive(Debug, PartialEq)]
pub struct CellView<'a> {
    coords: &'a Coord,
    color: &'a Color,
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

        cells.into_iter().for_each(|c| board.insert(c, red()));

        board.tick();

        assert_eq!(board.cell_map.get(&Coord(1, 1,)), None)
    }

    #[test]
    fn cell_comes_alive() {
        let cells = vec![Coord(0, 0), Coord(0, 1), Coord(0, 2)];

        let mut board = Board::new(100);

        cells.into_iter().for_each(|c| board.insert(c, red()));

        board.tick();

        board.cell_map.get(&Coord(1, 1)).expect("Should come alive");
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
