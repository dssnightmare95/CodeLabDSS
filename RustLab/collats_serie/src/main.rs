fn collatz_length(mut n: i32) -> u32{
    let mut length = 1;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }
    length
}

fn main() {
    let number: i32 = 2;
    let length = collatz_length(number);
    println!("The length of the Collatz sequence for {} is: {}", number, length);
}