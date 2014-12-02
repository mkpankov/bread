extern crate bread;

use bread::prepare;

fn main() {
    let input = "^fg(red)I'm red text. ^fg(blue)I am blue.";
    println!("{}", prepare(input.into_string()));
}
