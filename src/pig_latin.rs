use crate::input_processor;

pub fn convert() {
    println!("Type in a word (or sentence)");
    let user_input = input_processor::read_input();
    print!("The Pig Latin translation is: ");
    for word in user_input.split(' ') {
        print!("{} ", convert_word_to_pig_latin(word));
    }
    println!()
}

fn convert_word_to_pig_latin(word: &str) -> String {
    let first_letter = word.chars().next().unwrap();
    if is_vowel(first_letter) {
        return format!("{}-hay", word);
    }
    let mut translation = String::new();
    for (index, character) in word.chars().enumerate() {
        if index > 0 {
            translation.push(character);
        }
    }

    format!("{}-{}ay", translation, first_letter)
}


fn is_vowel(character: char) -> bool {
    "aeiou".contains(character)
}
