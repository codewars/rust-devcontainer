fn add(a: i32, b: i32) -> i32 {
    a + b
}

// -------------------- 8< --------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        assert_eq!(add(1, 1), 2);
    }
}
