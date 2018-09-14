// Globals are declared outside all other scopes
static  LANGUAGE   : &'static str = "Rust";
const   THRESHHOLD : i32          = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a 'const':
    // THRESHHOLD = 5;
}