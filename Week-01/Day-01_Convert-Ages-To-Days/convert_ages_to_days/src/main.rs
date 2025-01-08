use std::io;

fn main() {
    println!("Input your age: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input...");
    let input: u32 = input.trim().parse().expect("Was not a valid number");

    let age_in_days = calc_age(5);
    println!("If you are {input} years old, you've seen at least {age_in_days} days!");
}

fn calc_age(years: u32) -> u32 {
    years * 365
}
