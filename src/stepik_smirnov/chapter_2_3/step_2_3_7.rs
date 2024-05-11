// выведите сумму цифр в числе

use std::io;
// use crate::stepik_smirnov::utils_dir::utils_file::utils_mod::read_integer_from_stdin;

pub fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {
    let mut n = read_integer_from_stdin();
    let mut result  = 0;

    loop
    {
        if n == 0 {
            break
        }
        // println!("{} {}", n, result);
        result = result + (n % 10);
        n = (n - (n % 10)) / 10
        // println!("{} {}", n, result)
    }

    println!("{}", result);
}