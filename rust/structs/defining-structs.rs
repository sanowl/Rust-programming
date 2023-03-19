// Define a struct named `Rectangle`
struct Rectangle {
    // Define two fields of type `u32` named `width` and `height`
    width: u32,
    height: u32,
}

fn main() {
    // Create a new instance of `Rectangle` with `width` of `30` and `height` of `50`
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // Print the value of the `width` field of the `rect` instance using the dot notation
    println!("Width: {}", rect.width);

    // Print the value of the `height` field of the `rect` instance using the dot notation
    println!("Height: {}", rect.height);
}
