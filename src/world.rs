use crate::lattice::Cell;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldCell {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl From<Cell> for WorldCell {
    fn from(c: Cell) -> Self {
        Self { x: c.x, y: c.y, z: c.z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PartInstanceId(pub u64);

#[derive(Debug)]
pub struct World {
    occupancy: HashMap<WorldCell, PartInstanceId>,
}

impl World {
    pub fn new() -> Self {
        Self {
            occupancy: HashMap::new(),
        }
    }

    pub fn is_occupied(&self, cell: WorldCell) -> bool {
        self.occupancy.contains_key(&cell)
    }

    pub fn occupancy_len(&self) -> usize {
        self.occupancy.len()
    }
}
