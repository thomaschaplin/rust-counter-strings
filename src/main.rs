mod lib;
use lib::generate;
use lib::get_count;

fn main() {
    let count = get_count();
    let counter_string = generate(count);

    println!("{}", counter_string);
}
