fn main() {
    // Declare a tuple
    let my_tuple = ("hello", 42, true);

    // Declare an array
    let my_array = [1, 2, 3, 4, 5];

    // Declare a vector
    let mut my_vector = vec![1, 2, 3];

    // Add an element to the vector
    my_vector.push(4);

    // Print the values of the tuple, array, and vector
    println!("my_tuple = {:?}", my_tuple);
    println!("my_array = {:?}", my_array);
    println!("my_vector = {:?}", my_vector);
}
