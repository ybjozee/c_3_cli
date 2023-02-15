use crate::input_processor::get_int_from_input;

pub fn start() {
    println!("=======================");
    println!("Welcome to the Rustacean Fibonacci calculator");
    println!("What is the value of n?");

    let n = get_int_from_input();

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