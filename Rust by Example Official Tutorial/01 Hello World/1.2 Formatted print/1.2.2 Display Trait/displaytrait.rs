
// Import the 'fmt' module
use std::fmt;


// Struct for which we will implement 'fmt::Display'
struct Structure(i32);

// Implement the Display trait in order to use the '{}' marker for the type
impl fmt::Display for Structure {
    // This trait requires 'fmt' with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only 'x' and 'y' are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign: char =
            if self.imag > 0.0f64
                { '+' }
            else
                { '-' };
        write!(f, "{0} {1}{2}i", self.real, sign, self.imag)
    }
}

fn main() {
    let s = Structure(63);
    println!("{}", s);

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small=small_range,
             big=big_range);

    let point = Point2D { x: 3.3, y: 7.2 };
    
    println!("Compare Points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);

    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Complex");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}