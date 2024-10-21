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

    main2(); // calling main2().
    main3();
}

// functions with return values
fn five() -> i32 {
    5
}

fn main2() {
    let x = five();

    println!("The value of x is: {x}");
}

fn main3() {
    let z = plus_one(5);

    println!("The value of z is: {z}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 //semicolon(;)이 있으면 function에서 값이 return 되지 않음.
}