use std::io::Read;

fn main() {}

fn safe() {
    let safe = ('🍏', '🪙', '🍐', '🪙', '🍊', '🪙', '🍓', '🪙', '🍒', '🪙');

    println!("{}, {}, {}, {}, {}", safe.0, safe.2, safe.4, safe.6, safe.8);
}

fn add_strings_to_tuple() {
    let mut i1 = String::new();
    let mut i2 = String::new();
    let mut i3 = String::new();
    let mut i4 = String::new();
    let mut i5 = String::new();

    std::io::stdin().read_line(&mut i1).unwrap();
    std::io::stdin().read_line(&mut i2).unwrap();
    std::io::stdin().read_line(&mut i3).unwrap();
    std::io::stdin().read_line(&mut i4).unwrap();
    std::io::stdin().read_line(&mut i5).unwrap();

    let t = (i1, i2, i3, i4, i5);
    println!("{:?}", t);
}

fn update_tuple() {
    let mut tup = (10.0, 5.0, -2.0, 100.0, 2000.0, 0.0);

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();
    tup.0 = input.trim().parse::<f64>().unwrap();
    input.clear();

    std::io::stdin().read_line(&mut input).unwrap();
    tup.1 = input.trim().parse::<f64>().unwrap();
    input.clear();

    std::io::stdin().read_line(&mut input).unwrap();
    tup.2 = input.trim().parse::<f64>().unwrap();
    input.clear();

    std::io::stdin().read_line(&mut input).unwrap();
    tup.3 = input.trim().parse::<f64>().unwrap();
    input.clear();

    std::io::stdin().read_line(&mut input).unwrap();
    tup.4 = input.trim().parse::<f64>().unwrap();
    input.clear();

    println!(
        "{}, {}, {}, {}, {}, {}",
        tup.0 as i32, tup.1 as i32, tup.2 as i32, tup.3 as i32, tup.4 as i32, tup.5 as i32
    );
}

fn destruct() {
    let tup = ("До", "деструктуризации", 3, 2, 1, 0, "пуск!");
    let (a, b, c, d, e, f, g) = tup;
    println!("{a}, {b}, {c}, {d}, {e}, {f}, {g}");
}

fn print_array_el() {
    let arr = [-1, 0, 1, 2, 30, 4, 500];

    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).unwrap();

    print!("{}", arr[inp.trim().parse::<usize>().unwrap()]);
}

fn create_add_and_print() {
    let mut input = String::new();
    let mut arr = [0.; 5];

    for i in 0..5 {
        std::io::stdin().read_line(&mut input).unwrap();
        arr[i] = input.trim().parse::<f64>().unwrap();
        input.clear();
    }

    std::io::stdin().read_line(&mut input).unwrap();
    println!("{:.2}", arr[input.trim().parse::<usize>().unwrap()]);
}

fn numeric_array_inspector() {
    let mut input = String::new();
    let mut arr = [0_usize; 5];

    for i in 0..5 {
        std::io::stdin().read_line(&mut input).unwrap();
        arr[i] = input.trim().parse::<usize>().unwrap();
        input.clear();
    }

    println!(
        "{}, {}, {}, {}, {}",
        arr[arr[0]], arr[arr[1]], arr[arr[2]], arr[arr[3]], arr[arr[4]]
    );
}

fn static_array_modifier() {
    let mut index_input = String::new();
    let mut num_input = String::new();

    std::io::stdin().read_line(&mut index_input).unwrap();
    std::io::stdin().read_line(&mut num_input).unwrap();

    let mut arr = [0; 10];
    arr[index_input.trim().parse::<usize>().unwrap()] = num_input.trim().parse::<i32>().unwrap();
    println!("{:?}", arr);
}

fn destruct_array() {
    let arr = [0, 1, 1, 2, 3, 5];

    let [a, b, c, d, e, f] = arr;

    println!("{a}, {b}, {c}, {d}, {e}, {f}");
}

fn neighbours() {
    let arr = [-5, 1, 8, 2, 30, 4000, 500000];

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let index = input.trim().parse::<usize>().unwrap();
    print!(
        "{}\n{}\n{}\n",
        arr[index - 1] + arr[index + 1],
        arr[index - 1] - arr[index + 1],
        arr[index - 1] * arr[index + 1]
    );
}

fn swap() {
    let mut arr = [-621.5, 11.1, 2.0, -7.123, 0.125, 0.0, 0.000051789];

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();
    let first = input.trim().parse::<usize>().unwrap();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let second = input.trim().parse::<usize>().unwrap();

    (arr[first], arr[second]) = (arr[second], arr[first]);

    println!("{:.9?}", arr);
}
