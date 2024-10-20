// Original functions: basis
// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// Adding Parameters
// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// // Multiple Parameters
// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// Statements & Expressions
fn main() {
    // y: statement
    let y = {
        // x: expression. Returns x+1. -> the value then bounds to y.
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}