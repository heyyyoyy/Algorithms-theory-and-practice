
fn fibonacci(n: u64) -> u64 {
    assert!(n <= 40);

    let mut fib = (0, 1);
    for _ in 1..n {
        fib = (fib.1, fib.0 + fib.1);
    }
    fib.1
}

fn main() {
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(40), 102334155);
    println!("Done")
}