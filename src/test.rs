extern crate bread;
extern crate term;

use bread::Token::{Fg, Bg, Bold, Dim, Italic, Underline, Blink, Standout,
                   Reverse, Secure,
                   Reset,
                   Literal};
use bread::Color;
use bread::{
    BLACK,
    BLUE,
    BRIGHT_BLACK,
    BRIGHT_BLUE,
    BRIGHT_CYAN,
    BRIGHT_GREEN,
    BRIGHT_MAGENTA,
    BRIGHT_RED,
    BRIGHT_WHITE,
    BRIGHT_YELLOW,
    CYAN,
    GREEN,
    MAGENTA,
    RED,
    WHITE,
    YELLOW,
};
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

    let tokens = vec![Fg(Some(RED)), Literal("I'm red".into_string()), Reset];
    match t {
        None => panic!("Couldn't get terminal"),
        Some(ref mut t) => render(t, tokens),
    }
}
