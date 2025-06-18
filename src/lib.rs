pub fn hello_world() -> String {
    "Hello, World!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        let result = hello_world();
        assert_eq!(result, "Hello, World!".to_string());
    }
}
