use std::io::{self, BufRead};

fn pisano(m: u64) -> u64{
    let (mut a, mut b, mut c) = (0, 1, 1);
    for i in 1..m * m + 1 {
        c = (a + b) % m;
        a = b;
        b = c;
        if (a == 0) & (b == 1) {
            return i
        }
    }
    0
}

fn fib_mod(n: u64, m: u64) -> u64 {
    let remainder = n % pisano(m);
    let (mut first, mut second, mut result) = (0, 1, remainder);
    for _ in 1..remainder {
        result = (first + second) % m;
        first = second;
        second = result;
    }
    result % m
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    println!("Enter two numbers: ");
    stdin.lock().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(str::parse::<u64>);
    let (n, m) = (numbers.next().unwrap().expect("Woops"),
                  numbers.next().unwrap().expect("Woops"));
    let res: u64 = fib_mod(n, m);
    println!("Result: {}", res);
}
