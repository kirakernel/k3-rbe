#[allow(dead_code)]
#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
    Divide,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
            Self::Divide => {
                if y != 0 {
                    return x / y;
                }

                panic!("attempt to divide {x} by zero");
            }
        }
    }
}

// Cretes a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
type Oprs = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
    let y = Oprs::Divide;
    let z = Operations::Subtract;

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

    println!("{} + {} = {}", 10, 5, x.run(10, 5));
    println!("{} - {} = {}", 10, 5, z.run(10, 5));
    println!("{} / {} = {}", 10, 5, y.run(10, 5));

    println!("{} / {} = {}", 10, 0, y.run(10, 0));
}
