use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct PartMeta {
    pub id: String,
    pub name: String,
    pub system: String,

    pub lattice: LatticeRef,
    pub connectors: ConnectorsRef,

    #[serde(default)]
    pub visual: Option<VisualRef>,
}

#[derive(Debug, Deserialize)]
pub struct LatticeRef {
    pub file: String,
    pub unit_xy: String,
    pub unit_z: String,
}

#[derive(Debug, Deserialize)]
pub struct ConnectorsRef {
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct VisualRef {
    pub mesh: String,
    pub materials: String,
}

pub fn load_part_meta<P: AsRef<Path>>(path: P) -> Result<PartMeta, String> {
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("failed to read part.toml: {}", e))?;

    toml::from_str(&content)
        .map_err(|e| format!("failed to parse part.toml: {}", e))
}
