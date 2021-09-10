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
        let mut build_string = String::from("*");

        let count_as_string = count.to_string();

        build_string.push_str(&count_as_string.chars().rev().collect::<String>());

        if build_string.len() > count {
            build_string = build_string[..count as usize].to_string();
        }

        counter_string.push_str(&build_string);

        count -= build_string.len();
    }

    println!("{}", counter_string.chars().rev().collect::<String>())
}
