fn main() {
    let x = 5;
    let y = 10;

    // An if statement to compare `x` and `y` and print the appropriate message
    if x < y {
        println!("x is less than y");
    } else if x > y {
        println!("x is greater than y");
    } else {
        println!("x is equal to y");
    }

    let is_raining = true;

    // An if statement to check the value of `is_raining` and print the appropriate message
    if is_raining {
        println!("It is raining outside.");
    } else {
        println!("It is not raining outside.");
    }

    let age = 20;

    // An if statement to check if `age` is greater than or equal to 18
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are not yet an adult.");
    }

    let number = 7;

    // An if statement with multiple conditions using logical operators
    if number > 0 && number < 10 {
        println!("The number is between 0 and 10.");
    } else if number >= 10 && number < 20 {
        println!("The number is between 10 and 20.");
    } else {
        println!("The number is outside the range of 0 to 20.");
    }
}
