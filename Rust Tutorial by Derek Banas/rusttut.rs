
use std::{i8, i16, i32, i64, u8, u16, u32, u64,
          isize, usize, f32, f64};

use std::io::stdin;

fn main() {
    // show_type_sizes();
    // show_age();
    // show_my_name();
    // is_it_true_game();
    // format_some_things();
    // basic_calculations();
    // looping();
    // stringstest1();
    inputtest1();
    // TODO https://youtu.be/U1EFgCNLDB8?t=25m47s
}

fn inputtest1() {
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
