#[macro_export]
macro_rules! hash_map {
    // Empty map case
    () => {
        std::collections::HashMap::new()
    };

    // Handle the key-value pairs with trailing comma
    ($($key:expr => $value:expr,)+) => {
        hash_map!($($key => $value),+)
    };

    // Handle the key-value pairs without trailing comma
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_empty_hash_map() {
        let map: HashMap<i32, bool> = hash_map!();
        assert!(map.is_empty());
    }

    #[test]
    fn test_hash_map_with_entries() {
        let map = hash_map!(
            42 => true,
            64 => false,
            128 => true
        );

        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&42), Some(&true));
        assert_eq!(map.get(&64), Some(&false));
        assert_eq!(map.get(&128), Some(&true));
        assert_eq!(map.get(&999), None);
    }

    #[test]
    fn test_hash_map_with_trailing_comma() {
        let map = hash_map!(
            1 => "one",
            2 => "two",
            3 => "three",
        );

        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&3), Some(&"three"));
    }

    #[test]
    fn test_hash_map_different_types() {
        let map = hash_map!(
            "key1" => 100,
            "key2" => 200
        );

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("key1"), Some(&100));
        assert_eq!(map.get("key2"), Some(&200));
    }

    #[test]
    fn test_hash_map_complex_types() {
        // Using strings as keys and more complex values
        // Test with tuples as values instead of mixed vector types
        let map = hash_map!(
            "point1" => (1, 2, 3),
            "point2" => (4, 5, 6)
        );

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("point1"), Some(&(1, 2, 3)));
        assert_eq!(map.get("point2"), Some(&(4, 5, 6)));
    }

    #[test]
    fn test_hash_map_with_vec() {
        // Test using a single type of vector elements
        let map = hash_map!(
            "numbers" => vec![1, 2, 3],
            "more_numbers" => vec![4, 5, 6]
        );

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("numbers"), Some(&vec![1, 2, 3]));
        assert_eq!(map.get("more_numbers"), Some(&vec![4, 5, 6]));
    }
}