// Выведите наибольший общий делитель двух чисел.
// На ввод подается 2 натуральных числа
// Гарантируется наличие ответа в тестах

use std::io;
// use crate::stepik_smirnov::utils_dir::utils_file::utils_mod::read_integer_from_stdin;

pub fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {
    let a = read_integer_from_stdin();
    let b = read_integer_from_stdin();
    let mut result  = 0;

    for i in 1..=a {
        if a % i == 0 && b % i == 0 {
            result  = i;
        }
        // println!("{} {} {}", n, i, n % i)
    }

    println!("{}", result);
}