fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    check_bool_return();
    check_divisible();
    using_if_in_let();
}

fn check_bool_return() {
    let number2 = 3;

    if number2 == 3 {
        println!("number was three");
    }

}

fn check_divisible() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn using_if_in_let() {
    let condition = false;
    let number = if condition { 5 } else { 666 };

    println!("The value of number is: {number}");
}