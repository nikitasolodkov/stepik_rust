use std::io;

pub fn main() {
    // println!("Введите число:");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Не удалось считать строку");
    let number1: i32 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Не удалось считать строку");
    let number2: i32 = input.trim().parse().unwrap();

    // Выводим квадрат числа на экран
    // println!("Вы ввели число: {}", number*number);
    // println!("Вы ввели число1: {}", number1);
    // println!("Вы ввели число2: {}", number2);
    // println!("Сумма чисел {number1} и {number2}: {}", number1+number2);

    println!("{}", number1+number2);
}