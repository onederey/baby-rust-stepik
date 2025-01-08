use std::{io::Read, ptr::null};

fn main() {}

fn sum() {
    let mut first_num = String::new();
    let mut second_num = String::new();

    std::io::stdin().read_line(&mut first_num).unwrap();
    std::io::stdin().read_line(&mut second_num).unwrap();

    println!(
        "{}",
        first_num.trim().parse::<i32>().unwrap() + second_num.trim().parse::<i32>().unwrap()
    )
}

fn m_table() {
    let mut first_num = String::new();
    let mut second_num = String::new();

    std::io::stdin().read_line(&mut first_num).unwrap();
    std::io::stdin().read_line(&mut second_num).unwrap();

    let first_num_parsed: i32 = first_num.trim().parse().unwrap();
    let second_num_parsed: i32 = second_num.trim().parse().unwrap();

    for i in 1..11 {
        println!("{} x {} = {}", first_num_parsed, i, first_num_parsed * i);
    }

    println!();

    for i in 1..11 {
        println!("{} x {} = {}", second_num_parsed, i, second_num_parsed * i);
    }
}

fn east_is_sensitive_matter() {
    let mut first_num = String::new();
    let mut second_num = String::new();

    std::io::stdin().read_line(&mut first_num).unwrap();
    std::io::stdin().read_line(&mut second_num).unwrap();

    let first_num_parsed: i32 = first_num.trim().parse().unwrap();
    let second_num_parsed: i32 = second_num.trim().parse().unwrap();

    println!(
        "На поле доступно еще {} кв.м свободного места",
        first_num_parsed % second_num_parsed
    );
}

fn wisdom_castle() {
    let mut num_1 = String::new();
    let mut num_2 = String::new();

    std::io::stdin().read_line(&mut num_1).unwrap();
    std::io::stdin().read_line(&mut num_2).unwrap();

    let num_1_parsed = num_1.trim().parse::<i32>().unwrap();
    let num_2_parsed = num_2.trim().parse::<i32>().unwrap();

    println!(
        "{num_1_parsed} + ({num_2_parsed}) = {}",
        num_1_parsed + num_2_parsed
    );
    println!(
        "{num_1_parsed} - ({num_2_parsed}) = {}",
        num_1_parsed - num_2_parsed
    );
    println!(
        "{num_1_parsed} * ({num_2_parsed}) = {}",
        num_1_parsed * num_2_parsed
    );
    println!(
        "{num_1_parsed} / ({num_2_parsed}) = {}",
        num_1_parsed / num_2_parsed
    );
    println!(
        "{num_1_parsed} % ({num_2_parsed}) = {}",
        num_1_parsed % num_2_parsed
    );
}

fn swap_with_template() {
    let mut num_1 = String::new();
    let mut num_2 = String::new();

    std::io::stdin().read_line(&mut num_1).unwrap();
    std::io::stdin().read_line(&mut num_2).unwrap();

    let mut num_1_parsed = num_1.trim().parse::<i8>().unwrap();
    let mut num_2_parsed = num_2.trim().parse::<i8>().unwrap();

    (num_1_parsed, num_2_parsed) = (num_2_parsed, num_1_parsed);

    println!("{num_1_parsed}\n{num_2_parsed}");
}

fn convert_num() {
    let stdin = std::io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();

    let num = input.trim().parse::<i32>().unwrap();

    println!("{0:#b}\n{0:#o}\n{0:#x}", num);
}

fn print_table_with_converted_num() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();

    std::io::stdin().read_line(&mut input_1).unwrap();
    std::io::stdin().read_line(&mut input_2).unwrap();

    let num_1 = input_1.trim().parse::<i32>().unwrap();
    let num_2 = input_2.trim().parse::<i32>().unwrap();

    // closure - замыкание - || { ... }
    let print = |operator: char, result: i32| {
        println!("{num_1:#b} {operator} {num_2:#b} = {result:#b}");
        println!("{num_1:#o} {operator} {num_2:#o} = {result:#o}");
        println!("{num_1:#x} {operator} {num_2:#x} = {result:#x}");
        println!();
    };

    print('+', num_1 + num_2);
    print('-', num_1 - num_2);
    print('*', num_1 * num_2);
    print('/', num_1 / num_2);
    print('%', num_1 % num_2);
}

fn change_precision() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();

    std::io::stdin().read_line(&mut input_1).unwrap();
    std::io::stdin().read_line(&mut input_2).unwrap();

    let num = input_1.trim().parse::<f64>().unwrap();
    let precision = input_2.trim().parse::<usize>().unwrap();

    println!("{0:.precision$}", num);
}

fn sum_with_cast() {
    let mut input_float = String::new();
    let mut input_int = String::new();

    std::io::stdin().read_line(&mut input_float).unwrap();
    std::io::stdin().read_line(&mut input_int).unwrap();

    let balance = input_float.trim().parse::<f64>().unwrap();
    let add_sum = input_int.trim().parse::<i32>().unwrap();

    println!("{:.1}", balance + add_sum as f64);
}

fn whole_and_fraction_parts() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let f = input.trim().parse::<f64>().unwrap();

    println!("{}", f as i64);
    println!("{:.3}", f - (f as i64) as f64);
}

fn print_operations_again() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();

    std::io::stdin().read_line(&mut input_1).unwrap();
    std::io::stdin().read_line(&mut input_2).unwrap();

    let num_1 = input_1.trim().parse::<f64>().unwrap();
    let num_2 = input_2.trim().parse::<f64>().unwrap();

    println!("{num_1} + ({num_2}) = {}", num_1 + num_2);
    println!("{num_1} - ({num_2}) = {}", num_1 - num_2);
    println!("{num_1} * ({num_2}) = {}", num_1 * num_2);
    println!("{num_1} / ({num_2}) = {:.3}", num_1 / num_2);
    println!("{num_1} % ({num_2}) = {:.3}", num_1 % num_2);
}

fn print_float_with_e_notation() {
    let mut input_1 = String::new();
    std::io::stdin().read_line(&mut input_1).unwrap();
    let num_1 = input_1.trim().parse::<f64>().unwrap();
    println!("{:E}", num_1);
}

fn print_kg_and_lbs() {
    let mut input_kg = String::new();
    let mut input_lbs = String::new();

    std::io::stdin().read_line(&mut input_kg).unwrap();
    std::io::stdin().read_line(&mut input_lbs).unwrap();

    let kg = input_kg.trim().parse::<f64>().unwrap();
    let lbs = input_lbs.trim().parse::<f64>().unwrap();

    println!("{kg} kg = {:.3} lbs", kg * 2.205_f64);
    println!("{lbs} lbs = {:.3} kg", lbs * 0.454_f64);
}

fn simple_bool_print() {
    let t = true;
    let f = false;

    println!("Вам нравится программировать ? {}", t);
    println!("Вы ленитесь делать задания ? {}", f);
}

fn simple_char_print() {
    let mut char_input = String::new();
    std::io::stdin().read_line(&mut char_input).unwrap();
    let char = char_input.trim().parse::<char>().unwrap();
    println!("Мое настроение: {char}");
}

fn simple_char_print_2() {
    let mut char_input = String::new();

    for i in 1..6 {
        std::io::stdin().read_line(&mut char_input).unwrap();
        print!("{}", char_input.trim());
        char_input = String::new();
    }
}
