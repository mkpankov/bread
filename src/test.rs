extern crate bread;
extern crate term;

use bread::render;

fn main() {
    //let input = "^fg(red)I'm red text. ^fg(blue)I am blue.";
    let input = "^fg(bright-green)^bg(blue)I'm bright green \
                 ^fg(bright-magenta)^bg(bright-black)^bold()I am bright magenta";
    let mut t = term::stdout();
    match t {
        None => panic!("Couldn't get terminal"),
        Some(ref mut t) => render(t, input).unwrap(),
    };
}
