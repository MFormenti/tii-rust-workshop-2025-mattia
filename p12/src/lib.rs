///
/// Adds 2 integers
///
/// ```
/// use p12::add;
/// let r = add(2,2);
/// assert_eq!(r,4);
/// ```
///

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(100, 200), 300);
    }
}