// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}

#[derive(Debug)]
enum Work {
    Civilian,
    Soldier,
}

#[derive(Debug)]
enum DifficultRustConcepts {
    Ownership(String, String),
    Slices,
}

fn main() {
    // Explicitly use each name so they are available without
    // manual scoping
    use crate::Status::{Poor, Rich};
    // Automatically use each name inside Work.
    use crate::Work::*;

    // Automatically use each name inside DifficultRustConcepts
    use crate::DifficultRustConcepts::*;

    println!("Status: {:?} and {:?}", Poor, Rich);
    println!("Work: {:?} and {:?}", Civilian, Soldier);

    println!(
        "DifficultRustConcepts: {:?} and {:?}",
        Ownership(String::from("References"), String::from("Borrowing")),
        Slices
    );

    // Equivalent to Status::Poor.
    let status = Poor;
    // Equivalent to Work::Civilian.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit use above.
        Rich => println!("The rich have lost of money and greedy!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work and fight!"),
        Soldier => println!("Soldiers fight and work!"),
    }
}
