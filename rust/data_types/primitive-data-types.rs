fn main() {
    // Rust has several primitive data types, including booleans, integers, and floating-point numbers.

    // Boolean
    let is_true: bool = true;
    let is_false = false; // Type inference is used here

    // Integer
    let integer: i32 = -42; // Signed 32-bit integer
    let unsigned_integer: u32 = 42; // Unsigned 32-bit integer

    // Floating-point
    let float: f32 = 3.14; // Single-precision floating-point number
    let double: f64 = 3.141592653589793; // Double-precision floating-point number

    // Rust also has character and string types

    // Character
    let character: char = 'A';

    // String
    let string: String = String::from("Hello, world!");

    // Print the values of the variables
    println!("is_true = {}", is_true);
    println!("is_false = {}", is_false);
    println!("integer = {}", integer);
    println!("unsigned_integer = {}", unsigned_integer);
    println!("float = {}", float);
    println!("double = {}", double);
    println!("character = {}", character);
    println!("string = {}", string);
}
