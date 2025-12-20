pub fn hello() -> &'static str {
    "ssm"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_returns_expected_string() {
        assert_eq!(hello(), "ssm");
    }
}
