use std::io;

pub fn main() {
    let n = 3;
    let mut total = 0.0;

    // println!("Please Give Input: ");
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Не удалось считать строку");

        let number: f64 = input.trim().parse().unwrap();
        total = total + number;
    }

    let average = total / f64::from(n);
    // println!("total: {total}");
    // println!("average: {average}");
    println!("{average}");
}