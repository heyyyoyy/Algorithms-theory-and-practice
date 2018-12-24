
fn fib_digit(n: u32) -> u8 {
    assert!(n <= 10_u32.pow(7));
    let mut fib: (u8, u8) = (0, 1);
    for _ in 1..n {
        fib = (fib.1 % 10, (fib.0 + fib.1) % 10);
    }
    fib.1
}

fn main() {
    assert_eq!(fib_digit(10), 5);
    assert_eq!(fib_digit(55), 5);
    assert_eq!(fib_digit(210), 0);
    assert_eq!(fib_digit(1111), 9);
    println!("Done!")
}
