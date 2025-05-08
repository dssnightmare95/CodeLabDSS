fn sum(a: i32, b:i32) -> i32 {
    return a + b
}


fn main(){
    let x: i32 = 10;
    let y: i32 = 20;
    println!("Suma {x} + {y} = {}", sum(x, y));
}