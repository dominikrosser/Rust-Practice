fn main() {
    destructure_tuples();
    destructure_enums();
    destructure_and_dereference();
    destructure_struct();
}

/* tuples */

fn destructure_tuples() {
    let pair = (0, -2);

    println!("Tell me about {:?}", pair);
    match pair {
        (0, y) => println!("First is '0' and 'y' is '{:?}'", y),
        (x, 0) => println!("'x' is '{:?}' and last is '0'", x),
        _      => println!("It doesn't matter what they are"),
    }
}

/* enums */

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn fn destructure_enums() {
    let color = Color::RGB(122, 17, 40);

    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
    }
}

/* destructure and dereference */

fn destructure_and_dereference() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. 'mut_value': {:?}", m),
        }
    }
}

/* structs */

fn destructure_struct() {
    struct Foo { x: (u32, u32), y: u32 }

    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {}, y = {} ", a, b, y);

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // and you can also ignore some variables:
    let Foo { y, .. } = foo;
    println!("y = {}", y);
}
