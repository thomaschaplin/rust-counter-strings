use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut counter_string = String::new();

    let mut count: usize = match args[1].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(
            "Failed to read value \"{}\", we were expecting a number!",
            args[1].trim()
        ),
    };

    while count > 0 {
        let mut result = String::from("*");

        result.push_str(&count.to_string().chars().rev().collect::<String>());

        if result.len() > count {
            result = result[..count as usize].to_string();
        }

        counter_string.push_str(&result);

        count -= result.len();
    }

    println!("{}", counter_string.chars().rev().collect::<String>())
}
