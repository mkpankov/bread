extern crate term;

use State::{Beginning, Tag, Inside};
use Token::{Fg, Bg, Literal};
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
    Fg(Option<Color>),
    Bg(Option<Color>),
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
                                if current.as_slice() != "" {
                                    tokens.push(Literal(current.clone()));
                                }
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
                                let matched;
                                match current.as_slice() {
                                    "fg" | "bg" => matched = true,
                                    _ => return Err(format!("Expected fg or bg, found {}", current))
                                }
                                if matched {
                                    match current.as_slice() {
                                        "fg" => tokens.push(Fg(None)),
                                        "bg" => tokens.push(Bg(None)),
                                        _ => unreachable!(),
                                    }
                                    state = Inside;
                                    current = String::new();
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
                                    let maybe_last = tokens.pop();
                                    tokens.push(match maybe_last {
                                        None => return Err(format!("Expected a tag token in array, found {}", maybe_last)),
                                        Some(token) => {
                                            match token {
                                                Fg(_) => Fg(Some(color)),
                                                Bg(_) => Bg(Some(color)),
                                                _ => unreachable!(),
                                            }
                                        }
                                    });
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
                    &Fg(maybe_color) => {
                        term.fg(maybe_color.unwrap()).unwrap();
                    }
                    &Bg(maybe_color) => {
                        term.bg(maybe_color.unwrap()).unwrap();
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
    println!("{}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Fg(Some(term::color::RED)),
                  Literal("I'm red text ".into_string()),
                  Fg(Some(term::color::BLUE)),
                  Literal("I am blue".into_string())]))
}

#[test]
fn parse_fg_colors_bright() {
    let input = "^fg(bright-green)I'm bright green text ^fg(bright-magenta)I am bright magenta";
    println!("{}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Fg(Some(term::color::BRIGHT_GREEN)),
                  Literal("I'm bright green text ".into_string()),
                  Fg(Some(term::color::BRIGHT_MAGENTA)),
                  Literal("I am bright magenta".into_string())]))
}

#[test]
fn parse_fg_bg_colors() {
    let input = "^fg(bright-green)^bg(blue)I'm bright green text ^bg(bright-black)^fg(bright-magenta)I am bright magenta";
    println!("{}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Fg(Some(term::color::BRIGHT_GREEN)),
                  Bg(Some(term::color::BLUE)),
                  Literal("I'm bright green text ".into_string()),
                  Bg(Some(term::color::BRIGHT_BLACK)),
                  Fg(Some(term::color::BRIGHT_MAGENTA)),
                  Literal("I am bright magenta".into_string())]))
}
