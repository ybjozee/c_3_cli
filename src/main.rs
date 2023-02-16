use input_processor::get_user_input;

mod chorister;
mod employee_register;
mod fibonacci_calculator;
mod input_processor;
mod pig_latin;
mod rectangle;
mod statistics;
mod temperature_converter;
mod user_input;

fn main() {
    print_introduction();

    let mut user_input = get_user_input();

    while user_input.is_continue() {
        handle_menu_selection(user_input.get_selection());

        println!();
        print_instructions();

        user_input = get_user_input();
    }

    println!("Bye for now, please come again soon!");
}

fn print_introduction() {
    println!("=======================");
    println!("Welcome to the \"Control Flow Exercise\"");
    print_instructions();
}

fn print_instructions() {
    println!("Press 1 for the Rustacean Fibonacci calculator");
    println!("Press 2 for the Rustacean temperature converter");
    println!("Press 3 for the Rustacean Chorister");
    println!("Press 4 for the Rustacean Calculator");
    println!("Press 5 for the Rustacean Statistician");
    println!("Press 6 for the Rustacean Pig Latin Translator");
    println!("Press 7 for the Rustacean Employee Register");
    println!("Press q to exit");
}

fn handle_menu_selection(input: i8) {
    match input {
        1 => fibonacci_calculator::start(),
        2 => temperature_converter::start(),
        3 => chorister::sing(),
        4 => rectangle::print_statistics(),
        5 => statistics::print_statistics(),
        6 => pig_latin::convert(),
        7 => employee_register::start(),
        _ => {
            println!("I didn't quite catch that");
            print_instructions();
        }
    };
}