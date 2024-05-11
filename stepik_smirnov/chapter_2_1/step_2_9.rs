use std::io;

// Дано целое неотрицательное число N, определите число десятков в нем (предпоследнюю цифру числа).
// Если предпоследней цифры нет, то можно считать, что число десятков равно нулю. Числа 0 < N < 10^19
// Обратите внимание на тип данных, тут размер должен быть u128

fn read_integer_from_stdin() -> u128 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    input.trim().parse().expect("Введенное значение не является числом")
}

pub fn main() {

    let number: u128 = read_integer_from_stdin();

    println!("{}", (number % 100) / 10);
}

