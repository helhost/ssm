use crate::lattice::Cell;
use std::collections::HashMap;
use crate::loader::Part;

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
    next_id: u64,
}

impl World {
    pub fn new() -> Self {
        Self {
            occupancy: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn is_occupied(&self, cell: WorldCell) -> bool {
        self.occupancy.contains_key(&cell)
    }

    pub fn occupancy_len(&self) -> usize {
        self.occupancy.len()
    }

    pub fn place_part(&mut self, part: &Part, offset: WorldCell) -> Result<PartInstanceId, String> {
        let id = PartInstanceId(self.next_id);
        self.next_id += 1;

        let cells: Vec<WorldCell> = part
            .lattice
            .occupied
            .iter()
            .map(|c| WorldCell {
                x: c.x + offset.x,
                y: c.y + offset.y,
                z: c.z + offset.z,
            })
            .collect();

        for cell in &cells {
            if self.is_occupied(*cell) {
                return Err(format!("cell already occupied: {:?}", cell));
            }
        }

        for cell in cells {
            self.occupancy.insert(cell, id);
        }

        Ok(id)
    }
}
