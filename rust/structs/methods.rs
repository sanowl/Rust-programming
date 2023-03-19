// Define a struct named `Rectangle`
struct Rectangle {
    width: u32,
    height: u32,
}

// Define an implementation block for `Rectangle`
impl Rectangle {
    // Define a method named `area` that calculates the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Define a method named `perimeter` that calculates the perimeter of the rectangle
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    // Create a new instance of `Rectangle` with `width` of `30` and `height` of `50`
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // Call the `area` method on the `rect` instance and print the result
    let area = rect.area();
    println!("Area: {}", area);

    // Call the `perimeter` method on the `rect` instance and print the result
    let perimeter = rect.perimeter();
    println!("Perimeter: {}", perimeter);
}


//output 
// Area: 1500
// Perimeter: 160