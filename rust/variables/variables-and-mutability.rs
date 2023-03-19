fn main() {
    let x = 5; // Declare a variable `x` and assign it the value `5`
    println!("x = {}", x);

    // Attempt to reassign `x` to a new value, which will cause a compiler error
    // x = 6; // Uncomment this line to see the error message

    let mut y = 10; // Declare a mutable variable `y` and assign it the value `10`
    println!("y = {}", y);

    y = 20; // Reassign `y` to a new value, which is allowed because it's mutable
    println!("y = {}", y);
}


