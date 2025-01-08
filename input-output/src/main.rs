fn main() {}

fn hello_name() {
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("ошибка🤣");
    println!("Привет, {}!", name.trim());
}

fn marsianin() {
    let mut first_message = String::new();
    let mut second_message = String::new();
    let mut third_message = String::new();

    std::io::stdin().read_line(&mut first_message);
    std::io::stdin().read_line(&mut second_message);
    std::io::stdin().read_line(&mut third_message);

    print!("{first_message}{second_message}{third_message}");
}

fn lunohod() {
    let mut in_1 = String::new();
    let mut in_2 = String::new();
    let mut in_3 = String::new();
    let mut in_4 = String::new();

    std::io::stdin().read_line(&mut in_4);
    std::io::stdin().read_line(&mut in_3);
    std::io::stdin().read_line(&mut in_2);
    std::io::stdin().read_line(&mut in_1);

    println!(
        "{} {} {} {}",
        in_1.trim(),
        in_2.trim(),
        in_3.trim(),
        in_4.trim()
    );
}

fn arheolog() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    println!("{}", input.trim().parse::<i32>().expect("ошибка🤣"));
}
