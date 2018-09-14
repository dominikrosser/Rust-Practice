
use std::fmt;

struct MyVector(Vec<i32>);

impl fmt::Display for MyVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing
        // and create a reference to 'vec'.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over 'vec' in 'v' while enumarating the while enumerating the iteration
        // count in 'count'
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try! to return on errors.
            if count != 0 { write!(f, ", ")?; }

            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value
        write!(f, "]")
    }
}

fn main() {
    let v = MyVector(vec![232, 420, -3]);

    println!("{}", v);
}