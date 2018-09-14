
use std::{i8, i16, i32, i64, u8, u16, u32, u64,
          isize, usize, f32, f64};

use std::io::stdin;

fn main() {
    show_type_sizes();
    show_age();
    show_my_name();
    is_it_true_game();
    format_some_things();
    basic_calculations();
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
    // TODO https://youtu.be/U1EFgCNLDB8?t=10m8s
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
