// Import (via use) the fmt module to make it available.
use std::fmt;

// Define a structure for which fmt::Display will be implemented. This is
// a tuple struct named Structure that contains an i32.
struct Structure(i32);

// To use the {} marker, the trait fmt::Display must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires fmt with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the suplied output
        // stream: f. Returs fmt::Result which indicates whether the
        // operation succeeeded or failed. Note that write! uses syntax which
        // is very similar to println!.
        write!(f, "{}", self.0)
    }
}

// A structure holding two numbers. Debug will be derived so the results can be
// contrasted with Display.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement Display for MinMax
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use self.number to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct MyMinMax(i64, i64);

impl fmt::Display for MyMinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "from {} to {}.", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
#[allow(dead_code)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement Display for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only x and y are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    println!("{}", Structure(10));
    println!("{}", MinMax(-10, 10));
    println!("{:?}", MinMax(-10, 10));
    println!("{}", MyMinMax(-10, 10));
    println!("{:?}", MyMinMax(-10, 10));
    println!("{:?}", Point2D { x: 4.0, y: 5.5 });

    let point = Point2D { x: 1.5, y: 8.9 };
    println!("{}", point);
}
