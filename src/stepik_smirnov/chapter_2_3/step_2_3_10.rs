// Нужно найти все простые числа на отрезке a b.
// a,b - натуральные числа

use crate::stepik_smirnov::utils_dir::utils_file::utils_mod::read_integer_from_stdin;

pub fn main() {

    let mut a  = read_integer_from_stdin();
    let mut b  = read_integer_from_stdin();

    for i in a..=b {

        let mut result  = "Да";
        for j in 2..=i-1 {
            if i % j == 0 {
                result  = "Нет";
            }
        }
        if result == "Да" {
            println!("{}", i);
        }

    }

}