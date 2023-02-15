use crate::input_processor::{get_float_from_input, get_int_from_input};

pub fn start() {
    println!("=======================");
    println!("Welcome to the Rustacean temperature converter");
    println!("Press 1 for Celsius -> Fahrenheit");
    println!("Press 2 for Fahrenheit -> Celsius");

    let conversion_selection = get_int_from_input();

    println!("What is the temperature?");

    let temperature_input = get_float_from_input();

    let conversion;
    match conversion_selection {
        1 => {
            conversion = convert_celsius_to_fahrenheit(temperature_input);
            println!("{temperature_input} Celsius = {conversion} Fahrenheit");
        }
        2 => {
            conversion = convert_fahrenheit_to_celsius(temperature_input);
            println!("{temperature_input} Fahrenheit = {conversion} Celsius");
        }
        _ => println!("Unsupported selection provided"),
    };
}

fn convert_celsius_to_fahrenheit(input: f64) -> f64 {
    (input * 9.0 / 5.0) + 32.0
}

fn convert_fahrenheit_to_celsius(input: f64) -> f64 {
    (input - 32.0) * 5.0 / 9.0
}