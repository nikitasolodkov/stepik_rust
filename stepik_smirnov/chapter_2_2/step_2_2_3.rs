// Даны три натуральных числа A, B, C. Определите, существует ли треугольник с такими сторонами.
// Если треугольник существует, выведите строку YES, иначе выведите строку NO.
// Треугольник — это три точки, не лежащие на одной прямой.

use std::io;

fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {
    // println!("Введите три натуральных числа A, B, C:");
    let (a, b, c) = (read_integer_from_stdin(), read_integer_from_stdin(), read_integer_from_stdin());

    if (a + b > c) && (b + c > a) && (c + a > b) {
        println!("YES")
    }
    else {
        println!("NO")
    }
}