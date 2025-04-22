pub fn celsius2farnheit(celsius: i32) -> i32 {
    (celsius * 9 / 5) + 32
}

pub fn farnheit2celsius(farnheit: i32) -> i32 {
    (farnheit - 32) * 5 / 9
}

pub fn fibonacci_loop(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }

    c
}
pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}
