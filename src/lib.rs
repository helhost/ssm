pub mod part;
pub mod lattice;
pub mod connectors;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn loads_lego_3001_metadata() {
        let meta = part::load_part_meta("assets/parts/lego/3001/part.toml").unwrap();
        assert_eq!(meta.id, "lego:3001");
    }

    #[test]
    fn validates_lego_3001_lattice() {
        let content = fs::read_to_string("assets/parts/lego/3001/lattice.json").unwrap();
        let lattice: lattice::Lattice = serde_json::from_str(&content).unwrap();
        lattice.validate().unwrap();
        assert_eq!(lattice.occupied.len(), 24);
    }

    #[test]
    fn validates_lego_3001_connectors() {
        let content = fs::read_to_string("assets/parts/lego/3001/connectors.json").unwrap();
        let cf: connectors::ConnectorFile = serde_json::from_str(&content).unwrap();
        cf.validate().unwrap();
        assert_eq!(cf.connectors.len(), 8);
    }
}
