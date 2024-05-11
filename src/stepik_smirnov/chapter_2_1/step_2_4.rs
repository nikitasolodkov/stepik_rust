use std::io;

pub fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Не удалось считать строку");
    let celsius: f64 = input.trim().parse().unwrap();


    let fahrenheit: f64 = (celsius * f64::from(9.0/5.0)) + f64::from(32);

    println!("{fahrenheit}");

}