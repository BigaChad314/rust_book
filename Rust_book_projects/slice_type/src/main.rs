fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""
    println!("{}", word);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}", hello.to_owned()+world);

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // String을 byte array로 변환

    for (i, &item) in bytes.iter().enumerate() { // array를 반복하면서 index값을 tuple로 변환
        if item == b' ' { // b를 붙히는 이유: unicode가 아닌 byte-literal. (1바이트 공백)
            return i;
        }
    }

    s.len()
}
