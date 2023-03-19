fn main() {
    let x = 5;  // Declare a variable `x` and assign it the value `5`

    {
        let x = "hello";  // Declare a new variable `x` with the same name as the previous `x` variable
        println!("inner x = {}", x);
    }

    println!("outer x = {}", x);  // Print the value of the outer `x` variable
}
