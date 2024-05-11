// Выведите значение наименьшего нечетного элемента последовательности, а если в последовательности нет нечетных элементов - выведите число 0. Признак конца последовательности 0.

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

        if n % 2 == 1 && n < result || result == 0 {
            result = n;
        }
        if n == 0 {
            break
        }
    }

    println!("{}", result);
}



