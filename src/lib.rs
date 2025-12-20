pub mod part;
pub mod lattice;
pub mod connectors;
pub mod loader;

#[cfg(test)]
mod tests {
    use super::loader;

    #[test]
    fn loads_known_parts() {
        let cases = [
            ("assets/parts/lego/3001", "lego:3001", 24usize, 8usize),
            ("assets/parts/lego/3002", "lego:3002", 18usize, 6usize),
        ];

        for (path, id, occ, conns) in cases {
            let part = loader::load_part_dir(path).unwrap();
            assert_eq!(part.meta.id, id);
            assert_eq!(part.lattice.occupied.len(), occ);
            assert_eq!(part.connectors.connectors.len(), conns);
        }
    }
}


