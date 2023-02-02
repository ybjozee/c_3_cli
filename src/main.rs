use std::io;

fn main() {
    print_introduction();

    let mut user_input = read_input();

    //I'm not handling non-integer input apart from "q" for now
    while user_input.trim().ne("q") {
        handle_menu_selection(get_int_from_input(String::from(user_input)));

        println!();
        print_instructions();

        user_input = read_input();
    }

    println!("Bye for now, please come again soon!");
}

fn print_introduction() {
    println!("=======================");
    println!("Welcome to the \"Control Flow Exercise\"");
    print_instructions();
}

fn print_instructions() {
    println!("Press 1 for the Rustacean temperature converter");
    println!("Press 2 for the Rustacean Fibonacci calculator");
    println!("Press 3 for the Rustacean Chorister");
    println!("Press 4 for the Rustacean Calculator");
    println!("Press q to exit");
}

fn handle_menu_selection(input: i8) {
    match input {
        1 => start_fibonacci_calculator(),
        2 => start_temperature_converter(),
        3 => start_chorister(),
        4 => print_rectangle_area(),
        _ => {
            println!("I didn't quite catch that");
            print_instructions();
        }
    };
}

fn start_fibonacci_calculator() {
    println!("=======================");
    println!("Welcome to the Rustacean Fibonacci calculator");
    println!("What is the value of n?");

    let n = get_int_from_input(read_input());

    match n {
        0 => println!("n cannot be 0"),
        1 => println!("0"),
        2 => println!("0, 1"),
        _ => {
            let mut placeholders = [0, 1];

            for placeholder in placeholders {
                print!("{placeholder}, ")
            }

            for term in 3..n + 1 {
                let result = placeholders[0] + placeholders[1];
                print!("{result}");
                placeholders[0] = placeholders[1];
                placeholders[1] = result;
                if term != n {
                    print!(", ")
                }
            }
            println!();
        }
    };
}

fn start_temperature_converter() {
    println!("=======================");
    println!("Welcome to the Rustacean temperature converter");
    println!("Press 1 for Celsius -> Fahrenheit");
    println!("Press 2 for Fahrenheit -> Celsius");

    let conversion_selection = get_int_from_input(read_input());

    println!("What is the temperature?");

    let temperature_input = get_float_from_input(read_input());

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
        _ => println!("Unsupported selection provided")
    };
}

fn get_int_from_input(input: String) -> i8 {
    let input: i8 = input.trim().parse().expect("Provided input should be a whole number");
    input
}

fn get_float_from_input(input: String) -> f64 {
    let input: f64 = input.trim().parse().expect("Provided input should be a number");
    input
}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

fn convert_celsius_to_fahrenheit(input: f64) -> f64 {
    (input * 9.0 / 5.0) + 32.0
}

fn convert_fahrenheit_to_celsius(input: f64) -> f64 {
    (input - 32.0) * 5.0 / 9.0
}

fn start_chorister() {
    //second argument in range is not included in loop
    for day in 1..13 {
        let suffix = get_suffix_for_day(day);
        println!("On the {day}{suffix} of Christmas \nMy true love sent to me");

        let mut day = day;
        while day >= 1 {
            let gift = get_gift_for_day(day as usize);
            println!("{gift}");
            day -= 1;
        }

        println!();
    }
}

fn get_gift_for_day(day: usize) -> String {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle-doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    String::from(gifts[day - 1])
}

fn get_suffix_for_day(day: u8) -> String {
    match day {
        1 => String::from("st"),
        2 => String::from("nd"),
        3 => String::from("rd"),
        _ => String::from("th")
    }
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.breadth
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.breadth > other.breadth && self.length > other.length
    }

    fn square(size: u32) -> Self {
        Self {
            length: size,
            breadth: size,
        }
    }
}

fn print_rectangle_area() {
    let rect1 = Rectangle {
        length: 30,
        breadth: 50,
    };

    println!("rect 1 is {:?}", rect1);
    println!("The area of the rectangle is {} square pixels", rect1.area());

    let scale = 2;
    let rect2 = Rectangle {
        length: 50,
        breadth: dbg!(30 * scale),
    };
    dbg!(&rect2);
    println!("rect2 can hold rect1 : {}", rect2.can_hold(&rect1));
    println!("rect1 can hold rect1 : {}", rect1.can_hold(&rect2));
    dbg!(Rectangle::square(90));
}