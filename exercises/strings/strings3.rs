// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// In real development, trim a string only needs to  return &str
// Because trim operation doesn't need to change the content of the original string
// Only needs to shorten the start and end pointer
fn trim_me(input: &str) -> String {
    // Remove whitespace from both ends of a string!
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // Add " world!" to the string! There's multiple ways to do this!

    // Compose always involves creating a new string
    // So allocations is unavoidable, we can return a value String
    // but we can avoid unnecessary allocations to improve performance
    // Which is crucial in performance-critical code like kernel or game engine

    // concat! macro only applies to string literals which is at compile time

    // format! macro provides the worst performance
    // Because of the complexity of the format string

    // String::from() with plus operator  is the best choice for code that's not on the hot path
    // Since it may also involve second allocation
    // But much faster than format! macro

    // We can reserve enough space for the new string to avoid the second allocation
    // Which provides the best performance
    const WORLD: &str = " world!";
    let mut s = String::with_capacity(input.len() + WORLD.len());
    s.push_str(input);
    s.push_str(WORLD);

    s
}

fn replace_me(input: &str) -> String {
    // Replace "cars" in the string with "balloons"!
    // This operation will always involve creating a new string
    // So allocations is unavoidable, we can return a value String

    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
