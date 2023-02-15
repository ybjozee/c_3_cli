pub fn sing() {
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
        "Twelve drummers drumming",
    ];
    String::from(gifts[day - 1])
}

fn get_suffix_for_day(day: u8) -> String {
    match day {
        1 => String::from("st"),
        2 => String::from("nd"),
        3 => String::from("rd"),
        _ => String::from("th"),
    }
}