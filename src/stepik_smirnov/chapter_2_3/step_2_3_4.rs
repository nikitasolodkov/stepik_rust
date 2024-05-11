// Проверьте, является ли число n простым

use std::io;
// use crate::stepik_smirnov::utils_dir::utils_file::utils_mod::read_integer_from_stdin;

pub fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {
    let n = read_integer_from_stdin();

    let mut result  = "Да";
    for i in 2..=n-1 {
        if n % i == 0 {
            result  = "Нет";
        }
        // println!("{} {} {}", n, i, n % i)
    }
    println!("{}", result);
}