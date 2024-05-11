// Входные данные: Целое число.
// Выходные данные: "Четное" или "Нечетное".

use std::io;

fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {
    // println!("Введите число:");
    let number: i32 = read_integer_from_stdin();

    let result = if number % 2 == 0 { "Четное" } else { "Нечетное" };
    println!("{} ", result);
}