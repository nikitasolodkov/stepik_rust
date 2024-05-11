// Входные данные: Часы (целое число от 0 до 23).
// Утро [6-11), День[11-17), Вечер[17-22), Ночь[22-6)
// Выходные данные: "Утро", "День", "Вечер" или "Ночь" в зависимости от времени суток.

use std::io;

fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {
    // println!("Введите Часы (целое число от 0 до 23):");
    let hours: i32 = read_integer_from_stdin();

    match hours {
        0..=5 => println!("Ночь"),
        6..=10 => println!("Утро"),
        11..=16 => println!("День"),
        17..=21 => println!("Вечер"),
        22..=24 => println!("Ночь"),
        _ => println!("Другое значение"),
    }
}