use std::env;
use unicode_segmentation::UnicodeSegmentation;

pub fn generate(mut count: usize) -> String {
    let mut counter_string = String::new();

    while count > 0 {
        let mut result = String::from("*");

        result.push_str(&count.to_string().graphemes(true).rev().collect::<String>());

        if result.len() > count {
            result = result[..count as usize].to_string();
        }

        counter_string.push_str(&result);

        count -= result.len();
    }

    return counter_string.graphemes(true).rev().collect::<String>();
}

pub fn get_count() -> usize {
    let args: Vec<String> = env::args().collect();
    let count: usize = match args[1].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(
            "Failed to read value \"{}\", we were expecting a number!",
            args[1].trim()
        ),
    };
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        assert_eq!(generate(10), "*3*5*7*10*");
    }
}
