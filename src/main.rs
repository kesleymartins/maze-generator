use macroquad::prelude::*;
use chrono;

use lib::grid::Grid;

mod lib;

#[macroquad::main("window_conf")]
async fn main() {
    rand::srand(chrono::offset::Local::now().timestamp() as u64);

    let scale = 10f32;
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