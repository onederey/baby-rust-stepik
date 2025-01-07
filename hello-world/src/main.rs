use std::io;

fn main() {
    println!("Hello, world!");

    println!("Пожалуйста, введите свою строку:");

    let mut user_input = String::new();

    let result = io::stdin()
        .read_line(&mut user_input)
        .expect("Не удалось прочитать ввод!");

    println!("result = {}", result);

    println!("Вы ввели (trim): {}", user_input.trim());
    println!("Вы ввели (raw): {:#?}", user_input);

    let parsed_input_to_int = user_input
        .trim()
        .parse::<i32>()
        .expect("Пожалуйста, введите число!");

    println!("Вы ввели число: {}", parsed_input_to_int);
}
