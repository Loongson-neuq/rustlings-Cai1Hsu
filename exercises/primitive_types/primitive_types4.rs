// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // Rust just merged inclusive ranges in 1.80.0(quite recent)
    // see https://github.com/rust-lang/rust/issues/37854
    // so you can write: let nice_slice = &a[1..=3];
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
