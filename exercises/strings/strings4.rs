// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // string literals are string slices
    string_slice("blue");
    // to_string() creates a String instance
    string("red".to_string());
    // String::from creates a String instance
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());

    // The rule is simple:
    // - String literals or a slice of another string(&str or String or &String) are string slices
    // - String::from() or to_string() or to_owned() or into() or format!() always create a new String instance
    //    - Avoid call to_owned(), clone() as much as possible
    //    - You are probably just wasting memory
    // - When you have to modify/change the existing string, you need to create a new String instance
    //    - you don't have the ownership of the original string, so you can't change it, you have to create a new one
    //    - so it's always a String
    // - Trim operations doesn't change the string, we are just taking a slice of the original string so it's a string slice
}
