use std::io;

fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {

    println!("Введите число:");
    let mut number = String::new();
    io::stdin().read_line(&mut number)
        .expect("Не удалось считать строку");
    // input.trim().parse().expect("Введенное значение не является числом")
    let number: i32 = number.trim().parse().expect("Введенное значение не является числом");

    match number {
        1 => println!("Один"),
        2 => println!("Два"),
        3 | 4 => println!("Три или Четыре"),
        5..=10 => println!("Пять до Десяти"),
        _ => println!("Другое значение"),
    }
}