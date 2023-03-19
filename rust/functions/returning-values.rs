// Define a function named `add` that takes two `i32` arguments and returns an `i32` value

fn add(x: i32, y: i32) -> i32 {
    // Return the sum of `x` and `y`
    x + y
}

fn main() {
    // Call the `add` function with arguments `2` and `3` and print the result
    let result = add(2, 3);
    println!("Result: {}", result);
}
