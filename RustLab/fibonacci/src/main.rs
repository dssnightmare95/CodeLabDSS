fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

fn main () {
    let n: u32 = 20;
    let result = fibonacci(n);
    println!("Fibonacci of {} is {}", n, result);
}