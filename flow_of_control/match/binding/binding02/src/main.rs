fn some_number(n: u32) -> Option<u32> {
    Some(n)
}

fn main() {
    for i in 42..=45 {
        match some_number(i) {
            // Got `Some` variant, match if its value, bound to `n`,
            // is equal to 42.
            Some(n @ 42) => println!("The Answer: {}!", n),
            // Match any other number.
            Some(n) => println!("Not interesting... {}", n),
            // Match anything else (`None` variant).
            _ => (),
        }
    }
}
