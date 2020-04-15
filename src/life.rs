use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum CellEvent {
    Born(Cell),
    Died(Cell),
}

#[derive(Debug)]
pub struct Board {
    cell_map: HashMap<Coord, Color>,
    size: usize,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            size,
            cell_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, coord: Coord, color: Color) {
        self.cell_map.insert(coord, color);
    }

    // TODO: into vector
    pub fn cells(&self) -> Vec<CellView> {
        self.cell_map
            .iter()
            .map(|(coord, color)| CellView {
                coords: coord,
                color,
            })
            .collect::<Vec<CellView>>()
    }

    pub fn tick(&mut self) -> Vec<CellEvent> {
        let mut updates: Vec<CellEvent> = vec![];

        for (coords, color) in self.cell_map.iter() {
            let neighbours = self.cell_map.get_neighbours(coords);
            if neighbours.len() >= 2 && neighbours.len() <= 3 {
                // updates.push(CellEvent::Born(Cell {
                //     coords: (*coords).clone(),
                //     color: (*color).clone(),
                // }))
            } else {
                updates.push(CellEvent::Died(Cell {
                    coords: (*coords).clone(),
                    color: (*color).clone(),
                }))
            }
        }

        for update in &updates {
            match update {
                CellEvent::Born(cell) => self
                    .cell_map
                    .insert(cell.coords.clone(), cell.color.clone()),
                CellEvent::Died(cell) => self.cell_map.remove(&cell.coords),
            };
        }

        updates
    }
}

trait Cells {
    fn get_neighbours(&self, coord: &Coord) -> Vec<CellView>;
}

impl Cells for HashMap<Coord, Color> {
    fn get_neighbours(&self, coord: &Coord) -> Vec<CellView> {
        let diffs: Vec<(i32, i32)> = vec![
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, -1),
            (-1, 1),
            (-1, 0),
            (-1, 1),
        ];

        diffs
            .iter()
            .filter_map(|(dx, dy)| {
                let (coords, color) = self.get_key_value(&Coord(coord.0 + dx, coord.1 + dy))?;

                Some(CellView { coords, color })
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coord(i32, i32);

#[derive(Debug, Clone, PartialEq)]
pub struct Color(f32, f32, f32);

#[derive(Debug, PartialEq)]
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

        assert_eq!(
            board.tick(),
            vec![CellEvent::Died(Cell {
                coords: Coord(1, 1),
                color: red()
            })]
        )
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
