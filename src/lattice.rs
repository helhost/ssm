use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Lattice {
    pub units: Units,
    pub bounds: Bounds,
    pub occupied: Vec<Cell>,
}

#[derive(Debug, Deserialize)]
pub struct Units {
    pub x: String,
    pub y: String,
    pub z: String,
}

#[derive(Debug, Deserialize)]
pub struct Bounds {
    pub x: [i32; 2],
    pub y: [i32; 2],
    pub z: [i32; 2],
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Bounds {
    pub fn contains(&self, c: &Cell) -> bool {
        (self.x[0] <= c.x && c.x <= self.x[1]) &&
        (self.y[0] <= c.y && c.y <= self.y[1]) &&
        (self.z[0] <= c.z && c.z <= self.z[1])
    }
}

impl Lattice {
    pub fn validate(&self) -> Result<(), String> {
        for c in &self.occupied {
            if !self.bounds.contains(c) {
                return Err(format!("cell out of bounds: {:?}", c));
            }
        }
        Ok(())
    }
}
