use std::fmt::{self, Binary, Display, Formatter};
use std::io::{self, Write};

#[derive(Debug)]
enum Game {
    Player,
}

#[derive(Debug)]
#[allow(unused_variables)]
struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

impl Display for Triangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.a, self.b, self.c)
    }
}

#[derive(Debug)]
struct Vector2D {
    x: isize,
    y: isize,
}

impl Display for Vector2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // The f value implements the Write trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Different traits allow different formas of ouput of a type. The meaning
// of this format is to print the magnitude of a vector.
impl Binary for Vector2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let magnitude = (self.x * self.x + self.y * self.y) as f64;
        let magnitude = magnitude.sqrt();

        let decimals = f.precision().unwrap_or(3);
        let string = format!("{magnitude:.decimals$}");
        f.pad_integral(true, "", &string)
    }
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
    // ...

    // Formatting traits
    let a = 3f32;
    let b = 4f32;
    let c = 5f32;
    let triangle = Triangle { a, b, c };
    println!("\n\n{:?}", triangle);

    println!("\n\n{}", triangle);

    let pythagorean_triple = Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    };
    let result = format!("\n\n{pythagorean_triple}");
    println!("{}", result);

    let myvector = Vector2D { x: 3, y: 4 };

    println!("\n\n{myvector}"); // => "(3, 4)"
    println!("{myvector:?}"); // => "Vector2D {x: 3, y:4}"
    println!("{myvector:10.3b}"); // => "     5.000"

    // fmt::Display vs fmt::Debug - https://doc.rust-lang.org/std/fmt/#fmtdisplay-vs-fmtdebug
    println!("\n\n{} {:?}", 3, 4);
    println!("{} {:?}", 'a', 'b');
    println!("{} {:?}", "foo\n", "bar\n");

    // Related macros - https://doc.rust-lang.org/std/fmt/#related-macros
    // write!
    let mut w = Vec::new();
    let a = write!(&mut w, "Hello {}!", "world");
    println!("\n\n{:?}", a); //
    println!("{:?}", w);

    // print!
    print!("\n\nHello {}!", "world");
    println!("I have a newline {}", "character at the end");
    println!("She is {:?}", "so cute and nice.");

    // eprint!
    eprint!("\n\nHello {}!", "world");
    eprintln!("I have a newline {}", "character at the end");
    eprintln!("She is {:?}", "so cute and nice.");

    // format_args!
    let mut some_writer = io::stdout();
    println!("\n\n");
    _ = write!(
        &mut some_writer,
        "{}",
        format_args!("print with a {}", "macro")
    );

    my_fmt_fn(format_args!(", or a {} too", "function"));
}

fn make_string(a: u32, b: &str) -> String {
    format!("{b} {a}")
}

fn my_fmt_fn(args: fmt::Arguments<'_>) {
    _ = write!(&mut io::stdout(), "{args}\n")
}
