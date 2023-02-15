use std::io;

use crate::user_input::UserInput;

pub fn get_int_from_input() -> i8 {
    let input: i8 = read_input()
        .trim()
        .parse()
        .expect("Provided input should be a whole number");
    input
}

pub fn get_float_from_input() -> f64 {
    read_input()
        .trim()
        .parse()
        .expect("Provided input should be a number")
}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

pub fn get_user_input() -> UserInput {
    let input = read_input();
    //I'm not handling non-integer input apart from "q" for now
    if input.trim().eq("q") {
        return UserInput::Quit;
    }
    let input: i8 = input
        .trim()
        .parse()
        .expect("Provided input should be a whole number");
    UserInput::MenuSelection(input)
}