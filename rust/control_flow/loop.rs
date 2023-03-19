fn main() {
    // A simple for loop that iterates from 1 to 5 (inclusive)
    for i in 1..=5 {
        println!("Value of i: {}", i);
    }

    // A loop that runs indefinitely until the user enters "exit"
    loop {
        println!("Enter a command (type 'exit' to quit):");

        // Read user input from the console
        let input = std::io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");

        // Remove trailing newline characters from the input
        let input = input.trim();

        // If the input is "exit", break out of the loop
        if input == "exit" {
            break;
        }

        // Print the input value back to the console
        println!("You entered: {}", input);
    }

    // A while loop that runs as long as the condition is true
    let mut j = 0;
    while j < 5 {
        println!("Value of j: {}", j);
        j += 1;
    }
}
