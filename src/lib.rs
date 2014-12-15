extern crate term;

use State::{Beginning, Tag, Inside, InsideColor, InsideBool};
use Token::{Fg, Bg, Bold, Dim, Italic, Underline, Blink, Standout,
            Reverse, Secure,
            Reset,
            Literal};
use term::{Terminal, WriterWrapper};
pub use term::color::Color as Color;
pub use term::color::{
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

pub type FullTerminal = Box<Terminal<WriterWrapper> + Send>;

#[deriving(Show)]
enum State {
    Beginning,
    Tag,
    Inside,
    InsideColor,
    InsideBool,
}

#[deriving(Show, PartialEq, Eq)]
pub enum Token {
    Fg(Option<Color>),
    Bg(Option<Color>),
    Bold,
    Dim,
    Italic(Option<bool>),
    Underline(Option<bool>),
    Blink,
    Standout(Option<bool>),
    Reverse,
    Secure,
    Reset,
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
                        InsideColor => return Err(format!("Expected ')', found EOF")),
                        InsideBool => return Err(format!("Expected ')', found EOF")),
                        Beginning => if current.as_slice() != "" {
                            tokens.push(Literal(current.clone()))
                        }
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
                                match current.as_slice() {
                                    "fg" => {
                                        tokens.push(Fg(None));
                                        state = InsideColor;
                                    }
                                    "bg" => {
                                        tokens.push(Bg(None));
                                        state = InsideColor;
                                    }
                                    "bold" => {
                                        tokens.push(Bold);
                                        state = Inside;
                                    }
                                    "dim" => {
                                        tokens.push(Dim);
                                        state = Inside;
                                    }
                                    "italic" => {
                                        tokens.push(Italic(None));
                                        state = InsideBool;
                                    }
                                    "underline" => {
                                        tokens.push(Underline(None));
                                        state = InsideBool;
                                    }
                                    "blink" => {
                                        tokens.push(Blink);
                                        state = Inside;
                                    }
                                    "standout" => {
                                        tokens.push(Standout(None));
                                        state = InsideBool;
                                    }
                                    "reverse" => {
                                        tokens.push(Reverse);
                                        state = Inside;
                                    }
                                    "secure" => {
                                        tokens.push(Secure);
                                        state = Inside;
                                    }
                                    "reset" => {
                                        tokens.push(Reset);
                                        state = Inside;
                                    }
                                    _ => return Err(format!("Expected a tag, found {}", current)),
                                }
                                current = String::new();
                            }
                            _   => {
                                  return Err(format!("Expected lowercase letter or '(', found {}. Current: {}, state: {}, tokens: {}", i, current, state, tokens));
                            }
                        },
                        Inside => {
                            if current.as_slice() != "" {
                                return Err(format!("Expected no arguments, found {}", current));
                            }
                            match *i {
                                ')' => {
                                    state = Beginning;
                                }
                                _   => {
                                    return Err(format!("Expected ')', found {}", *i));
                                }
                            }
                        }
                        InsideBool => {
                            match *i {
                                ')' => {
                                    state = Beginning;
                                    let value = match current.as_slice() {
                                        "true" => true,
                                        "false" => false,
                                        _ => return Err(format!("Expected bool, found {}", current)),
                                    };
                                    let maybe_last = tokens.pop();
                                    current = String::new();
                                    tokens.push(match maybe_last {
                                        None => return Err(format!("Expected a tag token in array, found {}", maybe_last)),
                                        Some(token) => match token {
                                            Italic(_) => Italic(Some(value)),
                                            Underline(_) => Underline(Some(value)),
                                            Standout(_) => Standout(Some(value)),
                                            _ => return Err(format!("Expected tag, found {}", token)),
                                        }
                                    })
                                }
                                _ => {
                                    state = InsideBool;
                                    current.grow(1, *i);
                                },
                            }
                        }
                        InsideColor => {
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
                                    state = InsideColor;
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

pub fn render(term: &mut FullTerminal, tokens: Vec<Token>) {
    for t in tokens.iter() {
        match t {
            &Literal(ref string) => write!(term, "{}", string).unwrap(),
            &Fg(maybe_color) => {
                term.fg(maybe_color.unwrap()).unwrap();
            }
            &Bg(maybe_color) => {
                term.bg(maybe_color.unwrap()).unwrap();
            }
            &Bold => {
                term.attr(term::attr::Bold).unwrap();
            }
            &Dim => {
                term.attr(term::attr::Dim).unwrap();
            }
            &Italic(maybe_value) => {
                term.attr(term::attr::Italic(maybe_value.unwrap())).unwrap();
            }
            &Underline(maybe_value) => {
                term.attr(term::attr::Underline(maybe_value.unwrap())).unwrap();
            }
            &Blink => {
                term.attr(term::attr::Blink).unwrap();
            }
            &Standout(maybe_value) => {
                term.attr(term::attr::Standout(maybe_value.unwrap())).unwrap();
            }
            &Reverse => {
                term.attr(term::attr::Reverse).unwrap();
            }
            &Secure => {
                term.attr(term::attr::Secure).unwrap();
            }
            &Reset => {
                term.reset().unwrap();
            }
        }
    }
    term.reset().unwrap();
}

pub fn render_str(term: &mut FullTerminal, s: &str) -> Result<(), String> {
    let maybe_tokens = parse(s);
    match maybe_tokens {
        Err(string) => return Err(string),
        Ok(tokens) => {
            Ok(render(term, tokens))
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

#[test]
fn parse_fg_bg_bold_colors() {
    let input = "^fg(bright-green)^bg(blue)^bold()I'm bold bright green text";
    println!("{}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Fg(Some(term::color::BRIGHT_GREEN)),
                  Bg(Some(term::color::BLUE)),
                  Bold,
                  Literal("I'm bold bright green text".into_string()),
                  ]))
}

#[test]
fn parse_dim() {
    let input = "^dim()I'm just dim text";
    println!("{}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Dim,
                  Literal("I'm just dim text".into_string()),
                  ]))
}

#[test]
fn parse_reset() {
    let input = "^fg(red)I'm just dim text^reset()";
    println!("{}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Fg(Some(term::color::RED)),
                  Literal("I'm just dim text".into_string()),
                  Reset,
                  ]))
}

#[test]
fn parse_italic() {
    let input = "^italic(true)I'm just dim text^italic(false)";
    println!("{}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Italic(Some(true)),
                  Literal("I'm just dim text".into_string()),
                  Italic(Some(false)),
                  ]))
}

#[test]
fn parse_underline() {
    let input = "^underline(true)I'm underlined text^underline(false)";
    println!("{}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Underline(Some(true)),
                  Literal("I'm underlined text".into_string()),
                  Underline(Some(false)),
                  ]))
}

#[test]
fn parse_blink() {
    let input = "^blink()I'm blinking text^reset()";
    println!("{}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Blink,
                  Literal("I'm blinking text".into_string()),
                  Reset,
                  ]))
}

#[test]
fn parse_standout() {
    let input = "^standout(true)I'm standing out text^standout(false)";
    println!("{}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Standout(Some(true)),
                  Literal("I'm standing out text".into_string()),
                  Standout(Some(false)),
                  ]))
}

#[test]
fn parse_reverse() {
    let input = "^reverse()I'm reversed text^reset()";
    println!("{}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Reverse,
                  Literal("I'm reversed text".into_string()),
                  Reset,
                  ]))
}

#[test]
fn parse_secure() {
    let input = "^secure()I'm secure text^reset()";
    println!("{}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Secure,
                  Literal("I'm secure text".into_string()),
                  Reset,
                  ]))
}
