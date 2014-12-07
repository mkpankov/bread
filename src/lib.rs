extern crate term;

use State::{Beginning, Tag, Inside};
use Token::{Fg, Literal};
use term::{Terminal, WriterWrapper};
use term::color::Color;

pub type FullTerminal = Box<Terminal<WriterWrapper> + Send>;

#[deriving(Show)]
enum State {
    Beginning,
    Tag,
    Inside,
}

#[deriving(Show, PartialEq, Eq)]
enum Token {
    Fg(Color),
    Literal(String),
}

fn parse(s: &str) -> Result<Vec<Token>, String> {
    let mut state = Beginning;
    let mut current = String::new();
    let mut tokens = vec![];
    let mut iter = s.chars().peekable();
    loop {
        {
            let n = iter.peek();
            match n {
                None => {
                    match state {
                        Tag => return Err(format!("Expected lowercase letter or '(', found EOF")),
                        Inside => return Err(format!("Expected ')', found EOF")),
                        Beginning => tokens.push(Literal(current.clone())),
                    }
                    break;
                }
                Some(i) =>
                    match state {
                        Beginning => match *i {
                            '^' => {
                                state = Tag;
                                tokens.push(Literal(current.clone()));
                                current = String::new();
                            },
                            _   => {
                                state = Beginning;
                                current.grow(1, *i);
                            },
                        },
                        Tag => match *i {
                            'a'...'z' | '-' => {
                                state = Tag;
                                current.grow(1, *i);
                            },
                            '(' => {
                                match current.as_slice() {
                                    "fg" => {
                                        state = Inside;
                                        current = String::new();
                                    }
                                    _ => return Err(format!("Expected fg, found {}", current))
                                }
                            }
                            _   => {
                                  return Err(format!("Expected lowercase letter or '(', found {}. Current: {}, state: {}, tokens: {}", i, current, state, tokens));
                            }
                        },
                        Inside => {
                            match *i {
                                ')' => {
                                    state = Beginning;
                                    let color = match current.as_slice() {
                                        "black" => term::color::BLACK,
                                        "blue" => term::color::BLUE,
                                        "bright-black" => term::color::BRIGHT_BLACK,
                                        "bright-blue" => term::color::BRIGHT_BLUE,
                                        "bright-cyan" => term::color::BRIGHT_CYAN,
                                        "bright-green" => term::color::BRIGHT_GREEN,
                                        "bright-magenta" => term::color::BRIGHT_MAGENTA,
                                        "bright-red" => term::color::BRIGHT_RED,
                                        "bright-white" => term::color::BRIGHT_WHITE,
                                        "bright-yellow" => term::color::BRIGHT_YELLOW,
                                        "cyan" => term::color::CYAN,
                                        "green" => term::color::GREEN,
                                        "magenta" => term::color::MAGENTA,
                                        "red" => term::color::RED,
                                        "white" => term::color::WHITE,
                                        "yellow" => term::color::YELLOW,

                                        _ => return Err(format!("Expected color name, found {}", current)),
                                    };
                                    tokens.push(Fg(color));
                                    current = String::new();
                                },
                                _ => {
                                    state = Inside;
                                    current.grow(1, *i);
                                },
                            }
                        },
                    }
            }
        }
        iter.next();
    }
    Ok(tokens)
}

pub fn render(term: &mut FullTerminal, s: &str) -> Result<(), String> {
    let maybe_tokens = parse(s);
    match maybe_tokens {
        Err(string) => return Err(string),
        Ok(tokens) => {
            for t in tokens.iter() {
                match t {
                    &Literal(ref string) => write!(term, "{}", string).unwrap(),
                    &Fg(color) => {
                        term.fg(color).unwrap();
                    }
                }
            }
            Ok(term.reset().unwrap())
        }
    }
}

#[test]
fn parse_fg_two_colors() {
    let input = "^fg(red)I'm red text ^fg(blue)I am blue";
    assert!(parse(input)
         == Ok(
             vec![Literal("".into_string()),
                  Fg(term::color::RED),
                  Literal("I'm red text ".into_string()),
                  Fg(term::color::BLUE),
                  Literal("I am blue".into_string())]))
}

#[test]
fn parse_fg_colors_bright() {
    let input = "^fg(bright-green)I'm bright green text ^fg(bright-magenta)I am bright magenta";
    assert!(parse(input)
         == Ok(
             vec![Literal("".into_string()),
                  Fg(term::color::BRIGHT_GREEN),
                  Literal("I'm bright green text ".into_string()),
                  Fg(term::color::BRIGHT_MAGENTA),
                  Literal("I am bright magenta".into_string())]))
}
