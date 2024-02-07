fn main() {
    // Usage
    let result = format!("Hello");
    println!("{}", result);

    let result = format!("Hello, {}!", "world");
    println!("{}", result);

    let result = format!("The number is {}", 1);
    println!("{}", result);

    let result = format!("{:?}", (3, 4));
    println!("{}", result);

    let result = format!("{value}", value = 4);
    println!("{}", result);

    let people = "Rustaceans";
    let result = format!("Hello {people}");
    println!("{}", result);

    let result = format!("{} {}", 1, 2);
    println!("{}", result);

    let result = format!("{:04}", 42);
    println!("{}", result);

    let result = format!("{:#?}", (100, 200));
    println!("{}", result);

    // Positional Parameters
    let result = format!("{} {} {}", 1, 2, 3);
    println!("{}", result);

    let result = format!("{2} {1} {0}", 1, 2, 3);
    println!("{}", result);

    let result = format!("{1} {} {0} {}", 1, 2); // => 2 1 1 2
    println!("{}", result);

    // Named parameters
    let result = format!("{argument}", argument = "test");
    println!("{}", result);

    let result = format!("{name} {}", 1, name = 2);
    println!("{}", result);

    let result = format!("{a} {c} {b}", a = "a", b = 'b', c = 3);
    println!("{}", result);

    let argument = 2 + 2;
    let result = format!("{argument}");
    println!("{}", result);

    println!("{}", make_string(927, "label"));

    // Formatting Parameters

    // Width
    // All of these print "Hello x     !"
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);
    let width = 5;
    println!("Hello {:width$}!", "x");

    // bits
    println!("{:0width$b}", 2, width = 20);
    println!("{:0width$b}", 5, width = 64);
    println!("{:0width$b}", 7, width = 128);
}

fn make_string(a: u32, b: &str) -> String {
    format!("{b} {a}")
}
