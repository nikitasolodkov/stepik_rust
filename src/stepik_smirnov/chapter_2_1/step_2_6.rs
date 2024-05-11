use std::io;

pub fn main() {

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Не удалось считать строку");
    let hours: i32 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Не удалось считать строку");
    let minutes: i32 = input.trim().parse().unwrap();

    println!("{}", hours * 60 + minutes);

}