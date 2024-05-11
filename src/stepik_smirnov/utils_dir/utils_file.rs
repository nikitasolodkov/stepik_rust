pub mod utils_mod {
    use std::io;

    pub fn read_integer_from_stdin() -> i32 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Не удалось считать строку");
        input.trim().parse().expect("Введенное значение не является числом")
    }
}
