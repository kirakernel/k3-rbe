#[derive(Debug)]
enum Game {
    Player,
}

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

    // Fill/Alignment: https://doc.rust-lang.org/std/fmt/#fillalignment
    let result = format!("Hello {:<5}!", "x");
    println!("{}", result);

    let result = format!("Hello {:-<5}!", "x");
    println!("{}", result);

    let result = format!("Hello {:n<5}!", "x");
    println!("{}", result);

    let result = format!("Hello {:0<5}!", "x");
    println!("{}", result);

    let result = format!("Hello {:^5}!", "x");
    println!("{}", result);

    let result = format!("Hello {:>5}!", "x");
    println!("{}", result);

    let result = format!("Hello {:n>5}!", "x");
    println!("{}", result);

    // [fill] `<` - the argument is left-aligned in width columns
    let result = format!("Hey {:-<10}!", "you");
    println!("{}", result);
    // [fill] `^` - the argument is center-aligned in width columns
    let result = format!("Hey {:-^10}!", "you");
    println!("{}", result);
    let result = format!("Hey {:-^11}!", "you");
    println!("{}", result);
    // [fill] `>` - the argument is right-alined in width columns
    let result = format!("Hey {:->10}!", "you");
    println!("{}", result);

    println!("Hello {:^15}!", format!("{:?}", Some("hi"))); // => "Hello   Some("hi")   !"

    println!("{:>5}!", "Amy");
    println!("{:>5}!", 45);
    println!("{:>5?}!", "Amy");
    println!("{:>5?}!", 45);
    println!("{:>5}!", format!("{:?}", "Amy"));
    println!("{:>5}!", format!("{:?}", 45));

    // Sign/`#`/`0`
    let result = format!("Hello {:+}!", 5);
    println!("{}", result);

    let result = format!("{:#x}!", 27);
    println!("{}", result);

    let result = format!("Hello {:05}!", 5);
    println!("{}", result);

    let result = format!("Hello {:05}!", -5);
    println!("{}", result);

    let result = format!("{:#010x}!", 27);
    println!("{}", result);

    let p_number = 5i8; // 0b0101
    let n_number = -5i8;

    // + flag
    println!("\n\n{:+}", p_number);
    println!("{:+}", n_number);

    // # flag
    println!("\n\n{:#08b}", p_number);
    println!("{:#08b}", n_number);
    println!("{:?}", Game::Player);
    println!("{:#?}", Game::Player);
    println!("{:#x}", 0x00ff);
    println!("{:#X}", 0x00ff);
    println!("{:#b}", 0x00ff);
    println!("{:#o}", 0x00ff);
    println!("{:#}", 0x00ff);
    println!("{}", 0x00ff);

    // Precision
    // For example, the following calls all print the same thing `Hello x is 0.01000:`
    // Hello {arg 0 ("x")} is {arg 1 (0.01) with precision specified inline (5)}
    println!("\n\nHello {0} is {1:.5}", "x", 0.01);

    // Hello {arg 1 ("x")} is {arg 2 (0.01) with precision specified in arg 0 (5)}
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);

    // Hello {arg 0 ("x")} is {arg 2 (0.01) with precision specified in arg 1 (5)}
    println!("Hello {0} is {2:.1$}", "x", 5, 0.01);

    // Hello {next arg -> arg 0 ("x")} is {second of next two args -> arg 2 (0.01) with precision
    //                          specified in first of next two args -> arg 1 (5)}
    println!("Hello {} is {:.*}", "x", 5, 0.01);

    println!(
        "\n\n{}, `{name:.*}` has 3 fractional digits",
        "Hello",
        3,
        name = 1234.56
    );
    println!(
        "{}, `{name:.*}` has 3 characters",
        "Hello",
        3,
        name = "1234.56"
    );
    println!(
        "{}, `{name:>8.*}` has 3 right-aligned characters",
        "Hello",
        3,
        name = "1234.56"
    );

    // Localization - https://doc.rust-lang.org/std/fmt/#localization
    // Rust doesn't support different locales
    println!("\n\nThe value is {} :(", 1.5);

    // Escaping - https://doc.rust-lang.org/std/fmt/#escaping

    let result = format!("\n\nHello {{}}"); // => "Hello {}"
    println!("{}", result);

    let result = format!("{{ Hello"); // =>  "{ Hello"
    println!("{}", result);

    let variable = "hehehe!!";
    println!("{{variable}}");
    println!("{variable}");
    // println!("{{}}", variable);

    // Syntax - https://doc.rust-lang.org/std/fmt/#syntax
}

fn make_string(a: u32, b: &str) -> String {
    format!("{b} {a}")
}
