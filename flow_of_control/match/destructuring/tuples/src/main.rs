fn main() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    // match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("last is `2` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
}
