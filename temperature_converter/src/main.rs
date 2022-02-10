use std::io;

fn main() {
    // convert given temperature in Celsius to Fahrenheit
    // 1. input: temperature in Celsius
    let mut celsius = String::new();
    // co to było mut? co to było String::new() ?
    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");
    // co to było &mut? co to było .expect()?
    let _celsius: usize = celsius
        .trim()
        .parse()
        .expect("Celsius entered was not a number");
    // co to było trim, parse?
    // 2. convert to Fahrenheit
    // 3. print the Fahrenheit
    println!("Hello, world! {}", celsius);
}
