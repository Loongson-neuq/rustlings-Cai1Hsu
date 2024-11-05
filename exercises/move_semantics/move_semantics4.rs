// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut vec = foo();

    println!("{} has length {} content `{:?}`", "vec1", vec.len(), vec);

    vec.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec.len(), vec);
}

// `foo()` does NOT take `vec: Vec<i32>` as argument
// You can not return a reference to a local variable
fn foo() -> Vec<i32> {
    let mut vec = vec![];

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
