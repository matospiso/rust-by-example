fn main() {
    // integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);  // here, i32 is important (overflow)

    // scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // use underscores to improve readability
    println!("One million is written as {}", 1_000_000u32);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // integers can be expressed using hex, oct or bin using prefixes `0x`, `0o`, `0b`
    // bitwise operations:
    // and
    println!("0011 AND 0101 is {0} ({0:04b} in binary)", 0b0011u32 & 0b0101);
    // or
    println!("0011 OR 0101 is {0} ({0:04b} in binary)", 0b0011u32 | 0b0101);
    // xor
    println!("0011 XOR 0101 is {0} ({0:04b} in binary)", 0b0011u32 ^ 0b0101);
    // shift left
    let small_integer: u32 = 3;
    println!("{small_integer} ({small_integer:08b}) << 5 is {0} ({0:08b} in binary)", small_integer << 5);
    // left shift
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // We need to tell the compiler the type of the literals we use.
}