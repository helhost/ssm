pub mod part;
pub mod lattice;
pub mod connectors;
pub mod loader;

#[cfg(test)]
mod tests {
    use super::loader;

    #[test]
    fn loads_lego_3001_part_dir() {
        let part = loader::load_part_dir("assets/parts/lego/3001").unwrap();
        assert_eq!(part.meta.id, "lego:3001");
        assert_eq!(part.lattice.occupied.len(), 24);
        assert_eq!(part.connectors.connectors.len(), 8);
    }
}
