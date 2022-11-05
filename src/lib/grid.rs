use macroquad::prelude::*;

use super::cell::Cell;
use super::neighbour::Neighbour;
use super::utils::*;

pub struct Grid{
    cells: Vec<Cell>,
    current: usize,
    next: Option<Neighbour>,
    cols: usize,
    rows: usize,
    stack: Vec<usize>
}

impl Grid {
    pub fn new(scale: f32, margin: f32) -> Self {
        let mut cells = Vec::new();

        let rows = ((screen_height() - margin) / scale) as usize;
        let cols = ((screen_width() - margin) / scale) as usize;

        for i in 0..rows {
            for j in 0..cols {
                let pos = Vec2::from_array([
                    (j as f32 * scale) + margin * 0.5,
                    (i as f32 * scale) + margin * 0.5
                ]);

                let cell = Cell::new(pos, scale, (i, j));
                cells.push(cell);
            }
        }

        Self { cells, current: 0, next: None, cols, rows, stack: Vec::new() }
    }

    pub fn setup(&mut self) {
        self.cells[self.current].current = true;
        self.cells[self.current].visited = true;
    }

    pub fn update(&mut self) {
        self.setup_next();

        match &self.next {
            Some(neighbour) => {
                self.cells[self.current].current = false;

                self.cells[neighbour.index].current = true;
                self.cells[neighbour.index].visited = true;

                self.cells[self.current].remove_wall(&neighbour.side, false);
                self.cells[neighbour.index].remove_wall(&neighbour.side, true);

                self.stack.push(self.current);

                self.current = neighbour.index;
                self.next = None;
            },
            None => {
                if self.stack.is_empty() {
                    return;
                }

                self.cells[self.current].current = false;
                
                self.current = self.stack.pop().unwrap();

                self.cells[self.current].current = true;
            }
        }
    }

    pub fn draw(& self) {
        for cell in &self.cells {
            cell.draw()
        }
    }

    fn setup_next(&mut self) {
        let mut neighbours = Vec::new();

        let coord = self.cells[self.current].coord;

        if coord.0 > 0 {
            match self.neighbour_index(coord.1, coord.0-1) {
                Some(index) => {
                    let neighbour = Neighbour::new(index, Side::TOP);
                    neighbours.push(neighbour)
                },
                None => {}
            }
        }

        if coord.1 > 0 {
            match self.neighbour_index(coord.1-1, coord.0) {
                Some(index) => {
                        let neighbour = Neighbour::new(index, Side::LEFT);
                    neighbours.push(neighbour)
                },
                None => {}
            }
        }

        match self.neighbour_index(coord.1+1, coord.0) {
            Some(index) => {
                    let neighbour = Neighbour::new(index, Side::RIGHT);
                    neighbours.push(neighbour)
            },
            None => {}
        }

        match self.neighbour_index(coord.1, coord.0+1) {
            Some(index) => {
                    let neighbour = Neighbour::new(index, Side::BOTTOM);
                    neighbours.push(neighbour)
            },
            None => {}
        }

        neighbours.retain(|neighbour| !self.cells[neighbour.index].visited);

        if neighbours.len() > 0 {
            let random_num = rand::gen_range(0, neighbours.len());
            
            self.next = Some(neighbours.remove(random_num));
            return;
        }
    }

    fn neighbour_index(&self, i: usize, j: usize) -> Option<usize> {
        if i > self.cols-1 || j > self.rows-1 {
            return None;
        }

        Some(i + j * self.cols)
    }
}
