
// На ввод подается последовательность натуральных чисел. Признак конца последовательности - 0.
// Выведите наибольший элемент.

use std::io;

pub fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {

    let mut result  = 0;

    loop
    {
        let mut n = read_integer_from_stdin();
        if n > result {
            result = n;
        }
        if n == 0 {
            break
        }
    }

    println!("{}", result);
}



