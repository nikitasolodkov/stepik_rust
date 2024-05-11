use std::io;

// Дано целое трехзначное число. Найдите сумму его цифр.

fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {

    let number: i32 = read_integer_from_stdin();

    let result: i32 = number / 100 + ((number / 10) % 10) + number % 10;

    println!("{}", result);
}

