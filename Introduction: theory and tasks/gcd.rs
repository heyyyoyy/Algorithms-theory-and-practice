
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while (a != 0) & (b != 0) {
        if a >= b {
            a = a % b;
        } else {
            b = b % a;
        }
    }
    std::cmp::max(a, b)
}


fn main() {
    assert_eq!(gcd(18, 35), 1);
    assert_eq!(gcd(14159572, 63967072), 4);
    println!("Done!")
}
