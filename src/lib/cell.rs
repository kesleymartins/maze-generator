use macroquad::prelude::*;
use super::utils::Side;

pub struct Cell {
    pub coord: (usize, usize),
    pub visited: bool,
    pub current: bool,
    walls: (bool, bool, bool, bool),
    rect: Rect,
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

    pub fn remove_wall(&mut self, side: &Side, inverse: bool) {
        if inverse {
            match side {
                Side::BOTTOM => self.walls.0 = false,
                Side::LEFT => self.walls.1 = false,
                Side::TOP => self.walls.2 = false,
                Side::RIGHT => self.walls.3 = false,
            }

            return;
        }
        
        match side {
            Side::TOP => self.walls.0 = false,
            Side::RIGHT => self.walls.1 = false,
            Side::BOTTOM => self.walls.2 = false,
            Side::LEFT => self.walls.3 = false,
        }
    }

    pub fn draw(&self) {
        let mut color = GRAY;

        if self.visited {
            color = LIME;
        }

        if self.current {
            color = BLACK;
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