// На ввод подается последовательность натуральных чисел. Признак конца последовательности - 0.
// Выведите среднее арифметическое последовательности не включая 0.

use std::io;

pub fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {

    let mut result: f64  = 0.0;
    let mut counter  = 0;

    loop
    {
        let mut n = read_integer_from_stdin();
        if n == 0 {
            break
        }
        counter = counter + 1;
        result = result + f64::from(n);


    }

    println!("{}", result / f64::from(counter));
}