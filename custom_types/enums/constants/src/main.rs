/**
 * const: An unchangeable value (the common case).
 * static: A possibly mutable variable with 'static lifetime.
 * The static lifetime is inferred and does not have to be specified.
 * Accessing or modifying a mutable static variable is unsafe.
 */

// Globals are decrared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a const.
    // THRESHOLD = 5;
    // LANGUAGE = "JavaScript";
}