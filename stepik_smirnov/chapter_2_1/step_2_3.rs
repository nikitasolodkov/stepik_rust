use std::io;

pub fn main() {
    // println!("Введите число:");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Не удалось считать строку");
    let price: f64 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Не удалось считать строку");
    let amount: f64 = input.trim().parse().unwrap();

    println!("{}", price * amount);
}