pub mod part;

#[cfg(test)]
mod tests {
    use super::part::load_part_meta;

    #[test]
    fn loads_lego_3001_metadata() {
        let meta = load_part_meta("assets/parts/lego/3001/part.toml")
            .expect("should load part metadata");

        assert_eq!(meta.id, "lego:3001");
        assert_eq!(meta.system, "lego");
    }
}
