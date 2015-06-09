#![cfg(not(test))]

extern crate bread;
extern crate term;

use bread::Token as T;
use bread::Token::Attribute as A;
use bread as B;
use bread::{render_str, render};

fn main() {
    let input =
        "^fg(bright-green)^bg(blue)I'm bright green.\n\
         ^fg(bright-magenta)^bg(bright-black)^bold()\
         I am bold bright magenta on bright-black background.\n\
         ^reset()^dim()I'm just dim\n\
         ^reset()^italic(true)I'm italic. ^italic(false)I'm not.\n\
         ^reset()^underline(true)I'm underlined. ^underline(false)I'm not.\n\
         ^reset()^blink()I blink\n\
         ^reset()^standout(true)I stand out. ^standout(false)I don't.\n\
         ^reset()^reverse()I'm reversed. ^reset()I'm not.\n\
         ^reset()^secure()I'm secure.^reset()\n\
         \n";
    let mut t = term::stdout();
    match t {
        None => panic!("Couldn't get terminal"),
        Some(ref mut t) => render_str(t, input).unwrap(),
    };

    let tokens = [A(B::ForegroundColor(B::RED)), T::Literal(format!("I'm red\n")), T::Reset];
    match t {
        None => panic!("Couldn't get terminal"),
        Some(ref mut t) => render(t, &tokens),
    }
}
