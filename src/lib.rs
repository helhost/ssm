pub mod part;
pub mod lattice;

#[cfg(test)]
mod tests {
    use super::part::load_part_meta;
    use super::lattice::Lattice;
    use std::fs;

    #[test]
    fn loads_lego_3001_metadata() {
        let meta = load_part_meta("assets/parts/lego/3001/part.toml")
            .expect("should load part metadata");
        assert_eq!(meta.id, "lego:3001");
    }

    #[test]
    fn validates_lego_3001_lattice() {
        let content = fs::read_to_string("assets/parts/lego/3001/lattice.json").unwrap();
        let lattice: Lattice = serde_json::from_str(&content).unwrap();
        lattice.validate().unwrap();
        assert_eq!(lattice.occupied.len(), 24);
    }
}
