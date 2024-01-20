fn main() {
    info(13);

    for number in 1..=20 {
        info(number);
    }

    let boolean = true;

    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}

fn info(number: u8) {
    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }
}
