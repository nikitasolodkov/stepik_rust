// Определение типа треугольника по углам
// Входные данные: Три угла треугольника (в градусах).
// Выходные данные: "Равносторонний", "Равнобедренный", "Произвольный", или "Не существует" в зависимости от углов.

use std::io;

fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {
    // println!("Введите три угла треугольника (в градусах):");
    let (a, b, c) = (read_integer_from_stdin(), read_integer_from_stdin(), read_integer_from_stdin());

    if (a + b + c) != 180 {
        println!("Не существует")
    }
    else if a == b && b == c {
        println!("Равносторонний")
    }
    else if a == b || b == c || c == a {
        println!("Равнобедренный")
    }
    else {
        println!("Произвольный")
    }
}