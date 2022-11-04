use macroquad::prelude::*;
use chrono;

struct Cell {
    rect: Rect,
    coord: (usize, usize),
    walls: (bool, bool, bool, bool),
    visited: bool,
    current: bool
}

impl Cell {
    pub fn new(pos: Vec2, scale: f32, coord: (usize, usize)) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, scale, scale),
            walls: (true, true, true, true),
            visited: false,
            current: false,
            coord
        }
    }

    pub fn draw(&self) {
        let mut color = GRAY;

        if self.visited {
            color = PURPLE;
        }

        if self.current {
            color = DARKPURPLE;
        }

        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);

        // TOP
        if self.walls.0 {
            draw_line(
                self.rect.x, 
                self.rect.y,
                self.rect.x + self.rect.w,
                self.rect.y, 
                1f32, 
                WHITE
            );
        }

        // RIGHT
        if self.walls.1 {
            draw_line(
                self.rect.x + self.rect.w, 
                self.rect.y,
                self.rect.x + self.rect.w,
                self.rect.y + self.rect.h, 
                1f32, 
                WHITE
            );
        }

        // BOTTOM
        if self.walls.2 {
            draw_line(
                self.rect.x + self.rect.w, 
                self.rect.y + self.rect.h,
                self.rect.x,
                self.rect.y + self.rect.h, 
                1f32, 
                WHITE
            );
        }

        // LEFT
        if self.walls.3 {
            draw_line(
                self.rect.x, 
                self.rect.y + self.rect.h,
                self.rect.x,
                self.rect.y, 
                1f32, 
                WHITE
            );
        }
    }
}



struct Grid{
    cells: Vec<Cell>,
    current: usize,
    next: Option<usize>,
    cols: usize,
    rows: usize
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

        Self { cells, current: 0, next: None, cols, rows }
    }

    pub fn setup(&mut self) {
        self.cells[self.current].current = false;
    }

    pub fn update(&mut self) {
        self.setup_next();

        match self.next {
            Some(next) => {
                self.cells[self.current].visited = true;
                self.cells[self.current].current = false;

                self.cells[next].current = true;

                self.current = next;
                self.next = None;
            },
            None => println!("Sem Proximo")
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
                Some(index) => neighbours.push(index),
                None => println!("no TOP")
            }
        }

        if coord.1 > 0 {
            match self.neighbour_index(coord.1-1, coord.0) {
                Some(index) => neighbours.push(index),
                None => println!("no LEFT")
            }
        }

        match self.neighbour_index(coord.1+1, coord.0) {
            Some(index) => neighbours.push(index),
            None => println!("no RIGHT")
        }

        match self.neighbour_index(coord.1, coord.0+1) {
            Some(index) => neighbours.push(index),
            None => println!("no BOTTOM")
        }

        neighbours.retain(|index| !self.cells[*index].visited);

        if neighbours.len() > 0 {
            let random_num = rand::gen_range(0, neighbours.len());
            self.next = Some(neighbours[random_num]);
            return;
        }

        self.next = None;
    }

    fn neighbour_index(&self, i: usize, j: usize) -> Option<usize> {
        if i > self.cols-1 || j > self.rows-1 {
            return None;
        }

        Some(i + j * self.cols)
    }
}

#[macroquad::main("Maze Generator")]
async fn main() {
    rand::srand(chrono::offset::Local::now().timestamp() as u64);

    let scale = 20f32;
    let margin = 20f32;
    
    let mut grid = Grid::new(scale, margin);

    grid.setup();

    loop {
        clear_background(LIGHTGRAY);

        grid.update();
        grid.draw();

        next_frame().await;
    }
}