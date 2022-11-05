use super::utils::*;

pub struct Neighbour {
    pub index: usize,
    pub side: Side, 
}

impl Neighbour {
    pub fn new(index: usize, side: Side) -> Self {
        Self { index, side }
    }
}
