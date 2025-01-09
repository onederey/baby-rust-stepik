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
