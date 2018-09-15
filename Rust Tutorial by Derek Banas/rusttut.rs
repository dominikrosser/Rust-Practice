
use std::{i8, i16, i32, i64, u8, u16, u32, u64,
          isize, usize, f32, f64};

fn main() {
    show_type_sizes();
    show_age();
    show_my_name();
    is_it_true_game();
    format_some_things();
    basic_calculations();
    looping();
    stringstest1();
    guessgame();
    arraytest();
    vectortest();
    tupletest();
    say_hello("Dominik");
    funcwithparamsandreturntest();
    functionbindingtest();
    closurestest();
    ownertest1();
    ownertest2();
    ftest000();
    structtest1();
    enumtypetest1();
}

fn enumtypetest1() {
    let hulk = Hero::Strong(100);
    let quicksilver = Hero::Fast;
    let spiderman = Hero::Info { name: "Spiderman".to_owned(), secret: "Peter Parker".to_owned() };

    get_info(&hulk);
    get_info(&quicksilver);
    get_info(&spiderman);
}

enum Hero {
    Fast,
    Strong(i32),
    Info { name: String, secret: String}
}

fn get_info(h: &Hero) {
    match *h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(i) => println!("Lifts {} tons", i),
        Hero::Info { ref name, ref secret } => println!("{} is {}", name, secret),
    }
}

fn structtest1() {
    let c1 = Circle { x: 10.0, y: 10.0, radius: 10.0 };

    println!("X: {}, Y: {}, R: {}", c1.x, c1.y, c1.radius);
    println!("Radius from get_radius: {}", get_radius(&c1));
    println!("X from member fn get_x: {}", c1.get_x());

    println!("Circle Area: {}", c1.area());

    let r1 = Rectangle { height: 10.0, width: 10.0 };

    println!("Rect Area : {}", r1.area());
}

struct Rectangle {
    height: f64,
    width: f64,
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self. height * self.width
    }
}

fn get_radius(circle: &Circle) -> f64 {
    circle.radius
}

impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}

fn ftest000() {
    let v2 = vec![1,2,3];

    println!("Sum of Vect: {}", sum_vect(&v2));
    println!("Vect : {:?}", v2);
}

fn sum_vect(v1: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(
        0,
        |mut sum, &x| {
            sum += x; sum
        }
    );
    return sum;
}

fn ownertest2() {
    let prim1 = 1;
    let prim2 = prim1;

    println!("prim1 : {}", prim1);
}

fn ownertest1() {
    let v1 = vec![1,2,3];
    let v2 = v1;

    // println!("v1[0] : {}", v1[0]);
}

fn closurestest() {
    let sum_nums = |x: i32, y: i32| x + y;
    println!("7 + 8 = {}", sum_nums(7, 8));

    let num_ten = 10;

    let add_10 = |x: i32| x + num_ten;
    println!("5 + 10 = {}", add_10(5));
}

fn functionbindingtest() {
    let sum = get_sum;
    println!("6 + 4 = {}", sum(6, 4));
}

fn funcwithparamsandreturntest() {
    println!("5 + 4 = {}", get_sum(5, 4));
}

fn get_sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn say_hello(name: &str) {
    println!("Hello {}", name);
}

fn tupletest() {
    let mytuple = ("Dominik", 19);
    let mytuple_2: (&str, i8) = ("Mario", 17);

    println!("{}", mytuple_2.0);
}

fn vectortest() {
    let mut vec1 = vec![1,2,3,4,5];

    println!("Item 2 : {}", vec1[1]);

    for i in &vec1 {
        println!("Vect: {}", i);
    }

    vec1.push(6);

    vec1.pop();
}

fn arraytest() {
    let rand_array = [1,2,3];

    println!("{}", rand_array[0]);
    println!("{}", rand_array.len());
    println!("Second 2 : {:?}", &rand_array[1..3]);
}

fn guessgame() {
    'outer: loop {
        let number: i32 = 10;
        println!("Pick a number");

        loop {
            let mut line = String::new();
            let input = std::io::stdin().read_line(&mut line);

            let guess: Option<i32> = input.ok().map_or(
                None,
                |_| line.trim().parse().ok()
            );

            match guess {
                None => println!("Enter a Number"),
                Some(n) if n == number => {
                    println!("You guessed it!");
                    break 'outer;
                },
                Some(n) if n < number => println!("Too low"),
                Some(n) if n > number => println!("Too high"),
                Some(_) => println!("Error"),
            }
        }
    }
}

fn stringstest1() {
    let rand_string = "I am a random string";

    println!("Length: {}", rand_string.len());

    let (first, second) = rand_string.split_at(6);
    println!("First: {} Second: {}", first, second);

    let count = rand_string.chars().count();
    let mut chars = rand_string.chars();

    loop {
        match chars.next() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    let mut iter = rand_string.split_whitespace();

    loop {
        match iter.next() {
            Some(word) => println!("Word: {}", word),
            None => break,
        }
    }

    let rand_string_2 = "I am a random string\nThere are other strings like it\nThis string is the best";
    let mut lines = rand_string_2.lines();
    loop {
        match lines.next() {
            Some(line) => println!("Line: {}", line),
            None => break,
        }
    }

    println!("Find Best: {}", rand_string_2.contains("best"));
}

fn looping() {
    let mut x = 1;

    loop {
        if x % 2 == 0 {
            println!("{}", x);
            x += 1;
            continue;
        }
        if x > 10 { break; }
        x += 1;
        continue;
    }

    let mut y = 1;

    while y <= 10 {
        println!("{}", y);
        y += 1;
    }

    for z in 1..11 {
        println!("FOR: {}", z);
    }
}

fn basic_calculations() {
    println!("5 + 4 = {}", 5 + 4);
    println!("5 - 4 = {}", 5 - 4);
    println!("5 * 4 = {}", 5 * 4);
    println!("5 / 4 = {}", 5 / 4);
    println!("5 % 4 = {}", 5 % 4);

    let neg_4 = -4i32;
    println!("abs(-4) = {}", neg_4.abs());
    println!("4 ^ 6 = {}", 4i32.pow(6));
    println!("sqrt 9 = {}", 9f64.sqrt());
    println!("cbrt 27 = {}", 27f64.cbrt());
    println!("Round 1.45 = {}", 1.45f64.round());
    println!("Floor 1.45 = {}", 1.45f64.floor());
    println!("Ceiling 1.45 = {}", 1.45f64.ceil());
    println!("e ^ 2 = {}", 2f64.exp());
    println!("log(2) = {}", 2f64.ln());
    println!("log10(2) = {}", 2f64.log10());
    println!("90 to Radians = {}", 90.0f64.to_radians());
    println!("PI to Degrees = {}", 3.14f64.to_degrees());
    println!("Max 4, 5 = {}", 4f64.max(5f64));
    println!("Min 4, 5 = {}", 4f64.min(5f64));
    println!("Sin 3.14 = {}", 3.14f64.sin());
}

fn format_some_things() {
    println!("{:.2}", 1.234);
    println!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);
    println!("{ten:>ws$}", ten=10, ws=5);
    println!("{ten:>0ws$}", ten=10, ws=5);
}

fn is_it_true_game() {
    let is_it_true: bool = true;
    let let_x: char = 'x';

    println!("It is {0} that {1} is {0}", is_it_true, let_x);
}

fn show_my_name() {
    let (prename, surname) = ("Dominik", "Rosser");

    println!("My name is {1}, {0} {1}", prename, surname);
}

fn show_age() {
    let age: i32 = 40;
    println!("I am {} years old", age);
}

fn show_type_sizes() {
    println!("i8 | [{},{}]", i8::MIN, i8::MAX);
    println!("u8 | [{},{}]", u8::MIN, u8::MAX);
}
