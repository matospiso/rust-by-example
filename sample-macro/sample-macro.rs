// sample macro `say_hello`
// macros are created using `macro_rules!` macro
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument
    () => {
        // the macro will expand into the content of this block
        println!("I say hello")
    };
}

fn main() {
    // this call will expand into what is defined in the macro
    say_hello!()
}