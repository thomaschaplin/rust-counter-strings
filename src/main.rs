use std::env;
use std::fmt;

struct CounterString {
    count: usize,
}

impl CounterString {
    fn new(count: usize) -> Self {
        CounterString { count: count }
    }
}

impl fmt::Display for CounterString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let counter_string = CounterString::generate_counter_string(self.count);
        write!(f, "{}", counter_string)
    }
}

impl CounterString {
    fn get_count() -> usize {
        let args: Vec<String> = env::args().collect();
        let count: usize = match args[1].trim().parse() {
            Ok(num) => num,
            Err(_) => panic!(
                "Failed to read value \"{}\", we were expecting a number!",
                args[1].trim()
            ),
        };
        return count;
    }

    fn generate_counter_string(mut count: usize) -> String {
        let mut counter_string = String::new();

        while count > 0 {
            let mut result = String::from("*");

            result.push_str(&count.to_string().chars().rev().collect::<String>());

            if result.len() > count {
                result = result[..count as usize].to_string();
            }

            counter_string.push_str(&result);

            count -= result.len();
        }

        return counter_string.chars().rev().collect::<String>();
    }
}

fn main() {
    let count = CounterString::get_count();
    let counter_string = CounterString::new(count);

    println!("{}", counter_string);
}
