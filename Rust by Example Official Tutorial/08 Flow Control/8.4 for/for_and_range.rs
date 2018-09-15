fn main() {

    // Fizzbuzz with range for loop
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else {
            println!("{}", n);
        }
    }
}
