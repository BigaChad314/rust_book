fn main() {
    println!("Hello, world!");
}

// Building cargo: "cargo build" -> "./target/debug/hello_cargo"
// Running cargo: "cargo run"
// Checking the code: "cargo check" -> compile이 되는지 여부만 확인하여 훨씬 빠름.

// 정리:
// We can create a project using cargo new.
// We can build a project using cargo build.
// We can build and run a project in one step using cargo run.
// We can build a project without producing a binary to check for errors using cargo check.
// Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.