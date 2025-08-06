use std::io;

fn main() {
    println!("Press 1 for Celsius to Fareheit and 2 for vice versa");
    let mut sel = String::new();
    io::stdin().read_line(&mut sel).expect("Input failed");

    println!("Enter the temperature value");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Input failed");
    let temp: f32 = temp.trim().parse().expect("Parse failed");

    match sel.trim() {
        "1" => {
            let answer: f32 = (temp * (9.0 / 5.0)) + 32.0;
            println!("{temp}C is {answer}F");
        }
        "2" => {
            let answer: f32 = (temp - 32.0) * (5.0 / 9.0);
            println!("{temp}F is {answer}C");
        }
        _ => {
            println!("invalid Selection")
        }
    }
}
