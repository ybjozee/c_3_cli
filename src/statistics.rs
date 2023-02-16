use std::collections::HashMap;

pub fn print_statistics() {
    let numbers = vec![1, 2, 1, 2, 3, 5, 6, 7, 8, 5, 31, 24, 5, 5, 8];

    println!("\nNumbers are :");
    for number in &numbers {
        print!("{number} ")
    }
    println!("\nMedian value is {}", get_median(&numbers));
    println!("Mode is {}", get_mode(&numbers))
}

fn get_median(numbers: &Vec<i32>) -> i32 {
    let size = numbers.len();
    let mut numbers = numbers.to_vec();
    numbers.sort();

    println!("\nSorted numbers are :");
    for number in &numbers {
        print!("{number} ")
    }

    let median_index = if size % 2 == 0 { (size / 2) + 1 } else { size / 2 };

    println!("\nIndex of median number is {median_index}");

    if let Some(median_number) = numbers.get(median_index) {
        return *median_number;
    }
    panic!()
}

fn get_mode(numbers: &Vec<i32>) -> i32 {
    let mut frequencies = HashMap::new();
    for number in numbers {
        *frequencies.entry(number).or_insert(0) += 1;
    }

    let mut frequency_count: Vec<_> = frequencies.iter().collect();
    frequency_count.sort_by(|a, b| b.1.cmp(a.1));

    **frequency_count[0].0
}