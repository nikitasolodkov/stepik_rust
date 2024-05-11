// Напишите программу, которая решает уравнение ax^2 + bx + c = 0 и выводит корни на экран с
// использованием условных операторов. Обработайте все возможные случаи: два вещественных
// корня(вывести в порядке убывания), один вещественный корень, Нет корней
// На ввод подаются коэффициенты a, b, c

use std::io;

fn read_integer_from_stdin() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    return input.trim().parse().expect("Введенное значение не является числом");
}

pub fn main() {
    // println!("Введите три угла треугольника (в градусах):");
    let (a, b, c) = (read_integer_from_stdin(), read_integer_from_stdin(), read_integer_from_stdin());
    // D = b^2 — 4ac
    let discriminant: f64 = f64::from(i32::pow(b, 2)) - f64::from(4*a*c);

    if discriminant > 0.0 {
        // println!("Два вещественных корня (в порядке убывания)");
        let x1 = (- f64::from(b) - f64::sqrt(discriminant)) / f64::from(2 * a);
        let x2 = (- f64::from(b) + f64::sqrt(discriminant)) / f64::from(2 * a);
        println!("{} {}", x2, x1)
    } else if discriminant == 0.0 {
        // println!("Один вещественный корень");
        let x = -f64::from(b) / f64::from(2 * a);
        println!("{}", x)
    } else {
        println!("Нет корней")
    }
}