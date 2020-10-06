fn main() {
    println!("Fibonacci generator");
    println!("{}", fibonacci(0));
    println!("{}", fibonacci(1));
    println!("{}", fibonacci(2));
    println!("{}", fibonacci(3));
    println!("{}", fibonacci(4));
    println!("{}", fibonacci(5));
}

fn fibonacci (n: i32) -> i32 {
    if n == 0 {
        1
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
