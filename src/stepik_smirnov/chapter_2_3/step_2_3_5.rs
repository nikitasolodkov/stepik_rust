// Напишите программу, которая в последовательности натуральных чисел определяет максимальное число,
// кратное 5. Программа получает на вход количество чисел в последовательности, а затем сами числа.
// В последовательности всегда имеется число, кратное 5. Количество чисел не превышает 1000.
// Введённые числа не превышают 30 000. Программа должна вывести одно число - максимальное число,
// кратное 5

use std::io;
// use crate::stepik_smirnov::utils_dir::utils_file::utils_mod::read_integer_from_stdin;

pub fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {
    let n = read_integer_from_stdin();

    let mut result  = 0;
    for _ in 1..=n {
        let num = read_integer_from_stdin();
        if num % 5 == 0 && num > result {
            result  = num;
        }
        // println!("{} {} {}", n, i, n % i)
    }

    println!("{}", result);
}