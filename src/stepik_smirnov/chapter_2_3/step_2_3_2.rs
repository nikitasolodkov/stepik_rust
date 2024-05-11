// Выведите таблицу умножения к числу n
// 0 > n > 10

use crate::stepik_smirnov::utils_dir::utils_file::utils_mod::read_integer_from_stdin;

pub fn main() {
    println!("Введите число:");
    let n = read_integer_from_stdin();

    for i in 1..=10 {
        println!("{} x {} = {}", n, i, n*i)
    }
}