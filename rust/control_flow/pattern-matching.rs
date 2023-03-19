fn main() {
    // Define a variable named `number` and assign it a value of 3
    let number = 3;

    // Match the value of `number` with different patterns and execute the corresponding code block
    match number {
        1 => println!("The number is one"), // if the value of `number` is 1, print this message
        2 => println!("The number is two"), // if the value of `number` is 2, print this message
        3 => println!("The number is three"), // if the value of `number` is 3, print this message
        _ => println!("The number is something else"), // for any other value of `number`, print this message
    }

    // Define a variable named `letter` and assign it a value of 'c'
    let letter = 'c';

    // Match the value of `letter` with different patterns and execute the corresponding code block
    match letter {
        'a'..'z' => println!("The letter is a lowercase ASCII letter"), // if the value of `letter` is a lowercase ASCII letter, print this message
        'A'..'Z' => println!("The letter is an uppercase ASCII letter"), // if the value of `letter` is an uppercase ASCII letter, print this message
        _ => println!("The character is not an ASCII letter"), // for any other value of `letter`, print this message
    }

    // Define a variable named `person` and assign it a tuple containing the values "Alice" and 20
    let person = ("Alice", 20);

    // Match the value of `person` with different patterns and execute the corresponding code block
    match person {
        (name, age) if age >= 18 => println!("{} is an adult aged {}", name, age), // if the age of the person is greater than or equal to 18, print this message
        (name, age) => println!("{} is a minor aged {}", name, age), // for any other person, print this message
    }
}
