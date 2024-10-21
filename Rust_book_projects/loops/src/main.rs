fn main() {
    loop {
        println!("again!");
        break
    }
    // counting();
    // disambiguate_multiple_loops();
    conditional_while();
}

fn counting() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("counter inc.");
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn disambiguate_multiple_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 8 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn conditional_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}