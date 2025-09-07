fn main() {
    println!("Factorial of 5: {}", factorial(5));
    println!("Factorial of 10: {}", factorial(10));
}

fn factorial(n: u32) -> u32 {
    // Base case: factorial of 0 is 1
    if n == 0 { return 1; }
    // Recursive case: n * factorial(n-1)
    else { return n * factorial(n - 1); }
}