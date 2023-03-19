fn main() {
    // Declare a variable `x` and assign it the value `5`
    let x = 5;

    // Declare a variable `y` and assign it the value of `x` plus `1`
    let y = x + 1;

    // Declare a variable `name` and assign it a string value
    let name = "Alice";

    // Declare a variable `is_active` and assign it a boolean value
    let is_active = true;

    // Print the values of the variables
    println!("x = {}", x);
    println!("y = {}", y);
    println!("name = {}", name);
    println!("is_active = {}", is_active);
}


// The fn main() function is the entry point of the Rust program. This is where the program execution starts.

// We declare a variable x and assign it the value 5. The let keyword is used to declare a variable in Rust.

// We declare a variable y and assign it the value of x plus 1.
// This shows how we can use the value of an existing variable to compute the value of a new variable.


// We declare a variable name and assign it a string value "Alice".
// Note that the type of this variable is not explicitly specified, but Rust infers it as a &str type (string slice).


// We declare a variable is_active and assign it a boolean value true.


// We print the values of the variables to the console using the println! macro. The {} inside the string is a placeholder that will be replaced with the value of the corresponding variable.


// This will output the values of the variables to the console. You can try changing the values of the variables and see how the output changes. In Rust, variables are immutable by default, 
//so you cannot change their values once they are assigned.
// However, you can use the mut keyword to make them mutable.

