// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut x = 100;
    // the scope of a reference is the smallest scope that contains the reference
    // which means that the reference is valid until the last usage of the reference
    // You don't have to explicitly drop the reference or control the scope of the reference
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
