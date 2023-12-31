// Basic primitives

// Scalar types:
// - Signed ints: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (pointer size)
// - Unsigned ints: `u8` ... `u128`, `usize` (pointer size)
// - Floats: `f32`, `f64`
// - `char`: Unicode scalar values like 'a', '∞' (4 bytes)
// - `bool`: `true`, `false`
// - unit type `()` with single possible values `()` = empty tuple
//   despite being a tuple, it is not a compound type because it does not contain multiple values

// Compound Types:
// - arrays like `[1, 2, 3]`
// - tuples like `(1, true)`

fn main() {
    // Variable type annotation
    let logical: bool = true;  // regular
    let a_float: f64 = 1.0;  // regular
    let an_integer = 5i32;  // suffix
    // here default will be used 
    let default_float = 3.0;  // `f64`
    let default_integer = 5;  // `i32`
    // here type will be inferred
    let mut inferred_type = 12;  // `i64` will be inferred from another line
    inferred_type = 7483798982980i64;

    // mutable variable = value can be changed
    let mut mutable = 12;  // mutable `i32`
    mutable = 21;
    // This would cause an error -- type of variable can't be changed
    // mutable = 3.14;
    // Variables can be overwritten with shadowing:
    let mutable = 3.14;
}
