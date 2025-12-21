#![allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct UnitScale {
    pub xy: f32,
    pub z: f32,
}

impl UnitScale {
    pub const fn new(xy: f32, z: f32) -> Self {
        Self { xy, z }
    }

    pub const fn uniform(s: f32) -> Self {
        Self { xy: s, z: s }
    }

    pub fn cell_to_world(self, x: f32, y: f32, z: f32) -> [f32; 3] {
        [x * self.xy, y * self.xy, z * self.z]
    }
}

pub mod lego {
    use super::UnitScale;

    pub const STUD_MM: f32 = 7.8;
    pub const BRICK_H_MM: f32 = 9.6;

    pub const PLATE_PER_BRICK: f32 = 3.0;
    pub const PLATE_MM: f32 = BRICK_H_MM / PLATE_PER_BRICK;

    pub const SCALE_MM: UnitScale = UnitScale::new(STUD_MM, PLATE_MM);
    pub const SCALE_NORMALIZED: UnitScale = UnitScale::new(1.0, 1.0 / PLATE_PER_BRICK);
}
