use std::env;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let count = get_count();
    let counter_string = generate(count);

    println!("{}", counter_string);
}

pub fn generate(mut count: usize) -> String {
    let mut counter_string = String::new();

    while count > 0 {
        let mut result = String::from("*");
        result.push_str(&count.to_string().graphemes(true).rev().collect::<String>());

        if result.len() > count {
            result.truncate(count);
        }

        counter_string.push_str(&result);
        count -= result.len();
    }

    counter_string.graphemes(true).rev().collect::<String>()
}

pub fn get_count() -> usize {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a number as argument");
    }

    args[1].trim().parse().unwrap_or_else(|_| {
        panic!(
            "Failed to read value \"{}\", we were expecting a number!",
            args[1].trim()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        assert_eq!(generate(10), "*3*5*7*10*");
        assert_eq!(
            generate(50),
            "2*4*6*8*11*14*17*20*23*26*29*32*35*38*41*44*47*50*"
        );
        assert_eq!(generate(1), "*");
        assert_eq!(generate(0), "");
    }

    #[test]
    #[should_panic(expected = "Please provide a number as argument")]
    fn test_get_count_without_argument() {
        let _ = get_count();
    }

    #[test]
    #[should_panic(expected = "Please provide a number as argument")]
    fn test_get_count_with_invalid_argument() {
        env::set_var("ARG", "invalid");
        let _ = get_count();
    }
}
