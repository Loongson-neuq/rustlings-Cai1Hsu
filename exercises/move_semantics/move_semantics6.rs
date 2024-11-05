// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
// When you see 'not take ownership', it means you should use a reference
// And when you pass a stirng's immutable reference, you should always prefer to use `&str` instead of `&String`
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// When you see 'take ownership', it means you should not use a reference
// And when you pass a stirng's mutable reference, you should always prefer to use `&mut str` instead of `&mut String`
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
