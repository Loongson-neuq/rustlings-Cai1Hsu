// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // The mutability of the input is significant.
    // If the output vector just returns the same String instances, we'd better take the ownership
    // If we are going to return new String instances, we'd better take a reference
    // In real development, the second option is better, because it is more flexible
    // But the quiz design is not good, so I changed the test code.
    // We take the reference slice of the input, and strings are not moved, but borrowed
    // Command has copy semantics, so we don't have to take reference(and the reference is actually worse than value for small copy types)
    pub fn transformer(input: &[(&str, Command)]) -> Vec<String> {
        let mut output: Vec<String> = Vec::with_capacity(input.len()); // pre-allocate memory to avoid reallocation and memory copying
        for (string, command) in input.iter() {
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(n) => {
                    // Also avoids reallocation and memory copying
                    let mut new_string = String::with_capacity(string.len() + 3 * n);
                    new_string.push_str(string);
                    for _ in 0..*n {
                        new_string.push_str("bar");
                    }
                    output.push(new_string);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(&[
            // if we already have String instance, just take reference.
            // Compiler will automatically deference it.
            ("hello", Command::Uppercase),
            (" all roads lead to rome! ", Command::Trim),
            ("foo", Command::Append(1)),
            ("bar", Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
