// printing is handled by macros defined in `std::fmt`
// Examples:
// `format!` - write formatted text to String
// `print!` - same as `format!` but the text is printed to the console (`io::stdout`)
// `eprint!` - same as `print!` but to the standard error (`io::stderr`)
// `println!` - same as `print!` but new line is appended
// `eprintln!` - same as `eprint!` but new line is appended

fn main() {
    // generally `{}` will be automatically replaced with any arguments and stringified
    println!("{} days", 7);

    // positional arguments - just specify integer inside `{}` (start at 0)
    println!("May the {0} be with {2}. Not only {2} but also {1}.", "force", "him", "you");
    // rust checks if the correct number of arguments are used

    // named arguments
    println!("{subject} {verb} {object}.", 
        object="the cat",
        subject="The dog", 
        verb="chases");

    // different formating by specifying the format character after `:`
    println!("Base 10: {}", 123);
    println!("Base 2: {:b}", 123);
    println!("Base 8: {:o}", 123);
    println!("Base 16: {:x}", 123);
    println!("Base 16 with capital letters: {:X}", 123);

    // Justify:
    // right with width at least 5
    println!("{num:>5}", num=9);
    println!("{num:>5}", num=9999);
    println!("{num:>5}", num=999999); // this will will be wider than 5 characters
    // pad with specified sign (here `^`):
    // right
    println!("{num:^>5}", num=9);
    // left
    println!("{num:^<5}", num=9);
    // we can use named arguments here by appending a `$`
    println!("{num:^<width$}", num=9, width=8);

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);
    // This will not compile because `Structure` does not implement fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));

    // For Rust 1.58 and above, you can directly capture the argument from a surrounding variable.
    let num: f64 = 9.0;
    let width: usize = 5;
    println!("{num:>width$}");

    // format for debug (`fmt:Debug`) - `{:?}` marker
    println!("debugging {num:?}");

    // Activity: print Pi with specified number of decimals
    let pi: f64 = 3.1415926535;
    let decimals: usize = 4;
    println!("Pi is roughly {pi:.decimals$}");  // use `.`
}