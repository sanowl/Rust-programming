// Define a function named `calculate_square` that takes a single `i32` argument and returns an `i32` value
fn calculate_square(x: i32) -> i32 {
    // Calculate and return the square of `x`
    x * x
}

fn main() {
    // Call the `calculate_square` function with argument `5` and print the result
    let result = calculate_square(5);
    println!("Result: {}", result);
}
