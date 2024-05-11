// Выведите четные числа от 1 до n

use crate::stepik_smirnov::utils_dir::utils_file::utils_mod::read_integer_from_stdin;

pub fn main() {
    println!("Введите число:");
    let n = read_integer_from_stdin();

    for i in 1..=n {
        if i % 2 == 0 {
            println!("{}", i)
        }
    }
}