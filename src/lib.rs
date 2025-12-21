pub mod part;
pub mod lattice;
pub mod connectors;
pub mod loader;
pub mod world;
pub mod visuals;

#[cfg(test)]
mod tests {
    use super::loader;

    #[test]
    fn loads_known_parts() {
        let cases = [
            ("assets/parts/lego/3001", "lego:3001", 24usize, 8usize),
            ("assets/parts/lego/3002", "lego:3002", 18usize, 6usize),
            ("assets/parts/lego/3003", "lego:3003", 12usize, 4usize),
            ("assets/parts/lego/3004", "lego:3004", 6usize,  2usize),
        ];

        for (path, id, occ, conns) in cases {
            let part = loader::load_part_dir(path).unwrap();
            assert_eq!(part.meta.id, id);
            assert_eq!(part.lattice.occupied.len(), occ);
            assert_eq!(part.connectors.connectors.len(), conns);
        }
    }

    #[test]
    fn empty_world_has_no_occupancy() {
        let w = crate::world::World::new();
        assert_eq!(w.occupancy_len(), 0);
    }

    #[test]
    fn place_single_part() {
        let part = loader::load_part_dir("assets/parts/lego/3001").unwrap();
        let mut world = crate::world::World::new();

        let id = world
            .place_part(&part, crate::world::WorldCell { x: 0, y: 0, z: 0 })
            .unwrap();

        assert_eq!(world.occupancy_len(), 24);
        assert_eq!(id.0, 1);
    }

    #[test]
    fn overlapping_parts_fail() {
        let part = loader::load_part_dir("assets/parts/lego/3001").unwrap();
        let mut world = crate::world::World::new();

        world
            .place_part(&part, crate::world::WorldCell { x: 0, y: 0, z: 0 })
            .unwrap();

        let err = world
            .place_part(&part, crate::world::WorldCell { x: 0, y: 0, z: 0 })
            .unwrap_err();

        assert!(err.contains("occupied"));
    }
}
