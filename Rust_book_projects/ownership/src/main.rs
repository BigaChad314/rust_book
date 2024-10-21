fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`

    string_heap_move();
}

fn string_heap_move() {
    let s1 = String::from("this is how moving heap works");
    let s2 = s1;
    println!("{s2}");
}