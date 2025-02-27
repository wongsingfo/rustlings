// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

// https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files

#[macro_use]
mod macros {
    // #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
