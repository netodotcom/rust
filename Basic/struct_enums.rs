// Define a custom data type called Person with three fields: name, age, and height.
struct Person {
    name: String,
    age: u32,
    height: f32,
}

#[derive(Debug)]
// Define an enum called Color with three possible values: Red, Green, and Blue.
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
// Define an enum called Shape with two possible variants: Circle and Rectangle.
enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
}

// The main function is the entry point of the program.
fn main() {
    // Create a new Person instance with name, age, and height fields.
    let neto = Person {
        name: String::from("neto"),
        age: 25,
        height: 1.8,
    };
    
    // Create instances of the Color enum using it's variants.
    let color1 = Color::Red;
    let color2 = Color::Green;
    let color3 = Color::Blue;
    
    // Create instances of the Shape enum using it's variants and associated data.
    let shape1 = Shape::Circle(1.5);
    let shape2 = Shape::Rectangle(2.0, 3.0);
    
    // Print out the values of the Person instance and the Color and Shape enums using string interpolation and the Debug format specifier.
    println!("Name: {}, Age: {}, Height: {}", neto.name, neto.age, neto.height);
    println!("Color 1: {:?}", color1);
    println!("Color 2: {:?}", color2);
    println!("Color 3: {:?}", color3);
    println!("Shape 1: {:?}", shape1);
    println!("Shape 2: {:?}", shape2);
}
