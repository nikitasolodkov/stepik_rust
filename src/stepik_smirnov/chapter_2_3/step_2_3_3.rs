// Выведите факториал числа n (произведение всех чисел от 1 до n)

use crate::stepik_smirnov::utils_dir::utils_file::utils_mod::read_integer_from_stdin;

pub fn main() {
    let n = read_integer_from_stdin();
    println!("{}", (1..=n).product::<i32>());
}