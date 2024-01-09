fn main() {
    // Because of the annotation, the compiler knows that elem has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();

    // At this point the compiler doesn't know the exact type of vec, it
    // just knows that it's a vector of something (Vec<_>).

    // Insert elem in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that vec is a vector of u8s (Vec<u8>)

    println!("{:?}", vec);

    vec.push(10u8);
    vec.push(15u8);
    vec.push(14u8);
    vec.push(13u8);
    vec.push(16u8);

    println!("{:?}", vec);
}
