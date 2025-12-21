use crate::visuals::units::UnitScale;

#[derive(Clone, Copy, Debug)]
pub struct GridSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Wall {
    XMin,
    XMax,
    YMin,
    YMax,
    ZMin,
    ZMax,
}

#[derive(Clone, Copy, Debug)]
pub struct LineVertex {
    pub position: [f32; 3],
}

fn push_line(out: &mut Vec<LineVertex>, a: [f32; 3], b: [f32; 3]) {
    out.push(LineVertex { position: a });
    out.push(LineVertex { position: b });
}

fn p(scale: UnitScale, x: f32, y: f32, z: f32) -> [f32; 3] {
    scale.cell_to_world(x, y, z)
}

pub fn build_wall_grid(size: GridSize, scale: UnitScale, walls: &[Wall]) -> Vec<LineVertex> {
    let mut v = Vec::new();

    let x0 = 0.0_f32;
    let y0 = 0.0_f32;
    let z0 = 0.0_f32;

    let x1 = size.x as f32;
    let y1 = size.y as f32;
    let z1 = size.z as f32;

    for wall in walls {
        match wall {
            Wall::XMin => {
                let x = x0;
                for yi in 0..=size.y {
                    let y = yi as f32;
                    push_line(&mut v, p(scale, x, y, z0), p(scale, x, y, z1));
                }
                for zi in 0..=size.z {
                    let z = zi as f32;
                    push_line(&mut v, p(scale, x, y0, z), p(scale, x, y1, z));
                }
            }
            Wall::XMax => {
                let x = x1;
                for yi in 0..=size.y {
                    let y = yi as f32;
                    push_line(&mut v, p(scale, x, y, z0), p(scale, x, y, z1));
                }
                for zi in 0..=size.z {
                    let z = zi as f32;
                    push_line(&mut v, p(scale, x, y0, z), p(scale, x, y1, z));
                }
            }
            Wall::YMin => {
                let y = y0;
                for xi in 0..=size.x {
                    let x = xi as f32;
                    push_line(&mut v, p(scale, x, y, z0), p(scale, x, y, z1));
                }
                for zi in 0..=size.z {
                    let z = zi as f32;
                    push_line(&mut v, p(scale, x0, y, z), p(scale, x1, y, z));
                }
            }
            Wall::YMax => {
                let y = y1;
                for xi in 0..=size.x {
                    let x = xi as f32;
                    push_line(&mut v, p(scale, x, y, z0), p(scale, x, y, z1));
                }
                for zi in 0..=size.z {
                    let z = zi as f32;
                    push_line(&mut v, p(scale, x0, y, z), p(scale, x1, y, z));
                }
            }
            Wall::ZMin => {
                let z = z0;
                for xi in 0..=size.x {
                    let x = xi as f32;
                    push_line(&mut v, p(scale, x, y0, z), p(scale, x, y1, z));
                }
                for yi in 0..=size.y {
                    let y = yi as f32;
                    push_line(&mut v, p(scale, x0, y, z), p(scale, x1, y, z));
                }
            }
            Wall::ZMax => {
                let z = z1;
                for xi in 0..=size.x {
                    let x = xi as f32;
                    push_line(&mut v, p(scale, x, y0, z), p(scale, x, y1, z));
                }
                for yi in 0..=size.y {
                    let y = yi as f32;
                    push_line(&mut v, p(scale, x0, y, z), p(scale, x1, y, z));
                }
            }
        }
    }

    v
}
