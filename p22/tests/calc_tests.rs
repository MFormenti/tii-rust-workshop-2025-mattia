use p22::calc::{celsius2farnheit, farnheit2celsius, fibonacci_loop, fibonacci_rec};

#[test]
fn test_celsius_to_fahrenheit_conversion() {
    // Test freezing point
    assert_eq!(celsius2farnheit(0), 32);

    // Test boiling point
    assert_eq!(celsius2farnheit(100), 212);

    // Test room temperature
    assert_eq!(celsius2farnheit(20), 68);

    // Test negative values
    assert_eq!(celsius2farnheit(-40), -40); // -40 is the same in both scales
}

#[test]
fn test_fahrenheit_to_celsius_conversion() {
    // Test freezing point
    assert_eq!(farnheit2celsius(32), 0);

    // Test boiling point
    assert_eq!(farnheit2celsius(212), 100);

    // Test room temperature (approximately)
    assert_eq!(farnheit2celsius(68), 20);

    // Test negative values
    assert_eq!(farnheit2celsius(-40), -40); // -40 is the same in both scales
}

#[test]
fn test_celsius_fahrenheit_roundtrip() {
    // Test round-trip conversions for various temperatures
    let temperatures = [-30, -10, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    for temp in temperatures {
        // Due to integer division, we might lose some precision in the roundtrip
        // So we check if the difference is at most 1 degree
        let roundtrip = farnheit2celsius(celsius2farnheit(temp));
        assert!(
            (roundtrip - temp).abs() <= 1,
            "Round-trip conversion failed for {}°C: got {}°C",
            temp,
            roundtrip
        );
    }
}

#[test]
fn test_fibonacci_loop() {
    // First few Fibonacci numbers
    assert_eq!(fibonacci_loop(0), 0);
    assert_eq!(fibonacci_loop(1), 1);
    assert_eq!(fibonacci_loop(2), 1);
    assert_eq!(fibonacci_loop(3), 2);
    assert_eq!(fibonacci_loop(4), 3);
    assert_eq!(fibonacci_loop(5), 5);
    assert_eq!(fibonacci_loop(6), 8);
    assert_eq!(fibonacci_loop(7), 13);
    assert_eq!(fibonacci_loop(8), 21);
    assert_eq!(fibonacci_loop(9), 34);
    assert_eq!(fibonacci_loop(10), 55);
}

#[test]
fn test_fibonacci_recursive() {
    // First few Fibonacci numbers
    assert_eq!(fibonacci_rec(0), 0);
    assert_eq!(fibonacci_rec(1), 1);
    assert_eq!(fibonacci_rec(2), 1);
    assert_eq!(fibonacci_rec(3), 2);
    assert_eq!(fibonacci_rec(4), 3);
    assert_eq!(fibonacci_rec(5), 5);
    assert_eq!(fibonacci_rec(6), 8);
    assert_eq!(fibonacci_rec(7), 13);
    assert_eq!(fibonacci_rec(8), 21);
    assert_eq!(fibonacci_rec(9), 34);
    assert_eq!(fibonacci_rec(10), 55);

    // Note: We don't test larger values with the recursive version
    // because it would be inefficient and might cause a stack overflow
}

#[test]
fn test_fibonacci_implementations_match() {
    // Verify that both implementations produce the same results
    for n in 0..15 {
        assert_eq!(
            fibonacci_loop(n),
            fibonacci_rec(n),
            "Implementations differ at n={}",
            n
        );
    }
}

// This test checks for performance concerns with the recursive implementation
#[test]
#[ignore] // This test is marked as ignored by default as it might take too long
fn test_fibonacci_recursive_performance() {
    // This is a large enough value to be slow with the recursive implementation
    // but not so large that it would cause a stack overflow
    let n = 35;

    // This is still reasonable for the loop version
    let loop_result = fibonacci_loop(n);

    // The recursive version might be very slow for this value
    let rec_result = fibonacci_rec(n);

    assert_eq!(loop_result, rec_result);
}
