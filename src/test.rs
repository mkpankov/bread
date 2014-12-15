extern crate bread;
extern crate term;

use bread::Token as T;
use bread as B;
use bread::{render_str, render};

fn main() {
    let input = "^fg(bright-green)^bg(blue)I'm bright green \
                 ^fg(bright-magenta)^bg(bright-black)^bold()I am bright magenta \
                 ^reset()^dim()I'm just dim \
                 ^reset()^italic(true)I'm italic ^italic(false)I'm not \
                 ^reset()^underline(true)I'm underlined^underline(false)I'm not \
                 ^reset()^blink()I blink \
                 ^reset()^standout(true)I stand out^standout(false)I don't \
                 ^reset()^reverse()I'm reversed^reset()I'm not \
                 ^reset()^secure()I'm secure^reset()\
                 \n";
    let mut t = term::stdout();
    match t {
        None => panic!("Couldn't get terminal"),
        Some(ref mut t) => render_str(t, input).unwrap(),
    };

    let tokens = vec![T::Fg(Some(B::RED)), T::Literal("I'm red".into_string()), T::Reset];
    match t {
        None => panic!("Couldn't get terminal"),
        Some(ref mut t) => render(t, tokens),
    }
}
