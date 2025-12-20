use crate::connectors::ConnectorFile;
use crate::lattice::Lattice;
use crate::part::{load_part_meta, PartMeta};
use std::fs;
use std::path::Path;

pub struct Part {
    pub meta: PartMeta,
    pub lattice: Lattice,
    pub connectors: ConnectorFile,
}

pub fn load_part_dir<P: AsRef<Path>>(dir: P) -> Result<Part, String> {
    let dir = dir.as_ref();
    let meta_path = dir.join("part.toml");
    let meta = load_part_meta(&meta_path)?;

    let lattice_path = dir.join(&meta.lattice.file);
    let connectors_path = dir.join(&meta.connectors.file);

    let lattice_str = fs::read_to_string(&lattice_path)
        .map_err(|e| format!("failed to read {}: {}", lattice_path.display(), e))?;
    let lattice: Lattice = serde_json::from_str(&lattice_str)
        .map_err(|e| format!("failed to parse {}: {}", lattice_path.display(), e))?;
    lattice.validate()?;

    if lattice.units.x != meta.lattice.unit_xy
        || lattice.units.y != meta.lattice.unit_xy
        || lattice.units.z != meta.lattice.unit_z
    {
        return Err(format!(
            "lattice units mismatch: expected xy={}, z={} but got x={}, y={}, z={}",
            meta.lattice.unit_xy, meta.lattice.unit_z,
            lattice.units.x, lattice.units.y, lattice.units.z
        ));
    }

    let connectors_str = fs::read_to_string(&connectors_path)
        .map_err(|e| format!("failed to read {}: {}", connectors_path.display(), e))?;
    let connectors: ConnectorFile = serde_json::from_str(&connectors_str)
        .map_err(|e| format!("failed to parse {}: {}", connectors_path.display(), e))?;
    connectors.validate()?;

    Ok(Part { meta, lattice, connectors })
}
