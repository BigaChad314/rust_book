// parameter 앞에 "&"을 붙히면 reference를 하는 것.
// References are immutable by default. call 하는 것 외에 .push_str 같은 메서드로 변환시킬 수 없음.

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // length_calc_issue();
    // mutable_ref();
    tests_on_ref();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Previous version: runs fine, but has an issue.

fn length_calc_issue() {
    let s1 = String::from("ISSUE+WITH+RETURN");

    let (s2, len) = calculate_length_issue(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length_issue(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// mutable reference

fn mutable_ref() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn tests_on_ref() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

}