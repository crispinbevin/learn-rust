use std::io;

fn main() {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Enter a valid number");

    let number: u8 = number.trim().parse().expect("Try a lower number");

    if number < 1 {
        println!("the series is 1");
    } else {
        let mut a = 0;
        let mut b = 1;
        print!("The series is {a},{b},");

        for _ in 2..number {
            let next = a + b;
            print!("{next},");
            a = b;
            b = next;
        }
    }
}
