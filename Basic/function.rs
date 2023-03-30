fn is_x(input: &str) -> bool {
    if input == "x" {
        true 
    } else {
        false
    }
}

fn main() {
    // Define two variables, input1 and input2, with string values "x" and "y", respectively
    let input1 = "x";
    let input2 = "y";

    // Call the is_x function with input1 and input2 as arguments and print the results to the console
    println!("Is '{}' == 'x'? {}", input1, is_x(input1));
    println!("Is '{}' == 'x'? {}", input2, is_x(input2));
}
