extern crate term;

use State::{Beginning, Tag, Inside, InsideColor, InsideBool};
use Token::{Fg, Bg, Bold, Dim, Italic, Underline, Blink, Standout,
            Reverse, Secure,
            Reset,
            Literal,
            Partial,
};
use term::{StdTerminal};
use PartialToken as PT;
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

pub type FullTerminal = Box<StdTerminal>;

#[derive(Show)]
enum State {
    Beginning,
    Tag,
    Inside,
    InsideColor,
    InsideBool,
}

#[derive(Copy, Show, PartialEq, Eq)]
pub enum PartialToken {
    Fg,
    Bg,
    Italic,
    Underline,
    Standout,
}

#[derive(Show, PartialEq, Eq)]
pub enum Token {
    Partial(PartialToken),
    Fg(Color),
    Bg(Color),
    Bold,
    Dim,
    Italic(bool),
    Underline(bool),
    Blink,
    Standout(bool),
    Reverse,
    Secure,
    Reset,
    Literal(String),
}

fn parse(s: &str) -> Result<Vec<Token>, String> {
    let mut state = Beginning;
    let mut current = String::new();
    let mut tokens = vec![];
    for i in s.chars() {
        match state {
            Beginning => match i {
                '^' => {
                    state = Tag;
                    if &*current != "" {
                        tokens.push(Literal(current.clone()));
                    }
                    current = String::new();
                },
                _   => {
                    state = Beginning;
                    current.push(i);
                },
            },
            Tag => match i {
                'a'...'z' | '-' => {
                    state = Tag;
                    current.push(i);
                },
                '(' => {
                    match &*current {
                        "fg" => {
                            tokens.push(Partial(PT::Fg));
                            state = InsideColor;
                        }
                        "bg" => {
                            tokens.push(Partial(PT::Bg));
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
                            tokens.push(Partial(PT::Italic));
                            state = InsideBool;
                        }
                        "underline" => {
                            tokens.push(Partial(PT::Underline));
                            state = InsideBool;
                        }
                        "blink" => {
                            tokens.push(Blink);
                            state = Inside;
                        }
                        "standout" => {
                            tokens.push(Partial(PT::Standout));
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
                  _ => {
                      return Err(format!("Expected lowercase letter or '(', found {}. Current: {}, state: {:?}, tokens: {:?}", i, current, state, tokens));
                  }
                },
                Inside => {
                    if &*current != "" {
                        return Err(format!("Expected no arguments, found {}", current));
                    }
                    match i {
                        ')' => {
                            state = Beginning;
                        }
                    _   => {
                            return Err(format!("Expected ')', found {}", i));
                        }
                    }
                }
                InsideBool => {
                    match i {
                        ')' => {
                        state = Beginning;
                        let value = match &*current {
                            "true" => true,
                            "false" => false,
                            _ => return Err(format!("Expected bool, found {}", current)),
                        };
                        let maybe_last = tokens.pop();
                        current = String::new();
                        tokens.push(
                            match maybe_last {
                                None => return Err(format!("Expected a tag token in array, found {:?}", maybe_last)),
                                Some(token) => match token {
                                    Partial(PT::Italic) => Italic(value),
                                    Partial(PT::Underline) => Underline(value),
                                    Partial(PT::Standout) => Standout(value),
                                    _ => return Err(format!("Expected tag, found {:?}", token)),
                                }
                            })
                        }
                        _ => {
                            state = InsideBool;
                            current.push(i);
                        },
                    }
                }
                InsideColor => {
                    match i {
                        ')' => {
                        state = Beginning;
                        let color = match &*current {
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
                            None => return Err(format!("Expected a tag token in array, found {:?}", maybe_last)),
                            Some(token) => {
                                match token {
                                    Partial(PT::Fg) => Fg(color),
                                    Partial(PT::Bg) => Bg(color),
                                    _ => unreachable!(),
                                }
                            }
                        });
                        current = String::new();
                    },
                    _ => {
                        state = InsideColor;
                        current.push(i);
                    },
                }
            },

        }
    }
    match state {
        Tag => return Err(format!("Expected lowercase letter or '(', found EOF")),
        Inside => return Err(format!("Expected ')', found EOF")),
        InsideColor => return Err(format!("Expected ')', found EOF")),
        InsideBool => return Err(format!("Expected ')', found EOF")),
        Beginning => if &*current != "" {
            tokens.push(Literal(current.clone()))
        }
    }

    Ok(tokens)
}

pub fn render(trm: &mut FullTerminal, tokens: &[Token]) {
    for t in tokens.iter() {
        match *t {
            Literal(ref string) => write!(trm, "{}", string).unwrap(),
            Fg(color) => {
                trm.fg(color).unwrap();
            }
            Bg(color) => {
                trm.bg(color).unwrap();
            }
            Bold => {
                trm.attr(term::Attr::Bold).unwrap();
            }
            Dim => {
                trm.attr(term::Attr::Dim).unwrap();
            }
            Italic(value) => {
                trm.attr(term::Attr::Italic(value)).unwrap();
            }
            Underline(value) => {
                trm.attr(term::Attr::Underline(value)).unwrap();
            }
            Blink => {
                trm.attr(term::Attr::Blink).unwrap();
            }
            Standout(value) => {
                trm.attr(term::Attr::Standout(value)).unwrap();
            }
            Reverse => {
                trm.attr(term::Attr::Reverse).unwrap();
            }
            Secure => {
                trm.attr(term::Attr::Secure).unwrap();
            }
            Reset => {
                trm.reset().unwrap();
            }
            Partial(_) => unreachable!(),
        }
    }
    trm.reset().unwrap();
}

pub fn render_str(term: &mut FullTerminal, s: &str) -> Result<(), String> {
    let tokens = try!(parse(s));
    Ok(render(term, &*tokens))
}

#[test]
fn parse_fg_two_colors() {
    let input = "^fg(red)I'm red text ^fg(blue)I am blue";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Fg(term::color::RED),
                  Literal("I'm red text ".to_string()),
                  Fg(term::color::BLUE),
                  Literal("I am blue".to_string())]))
}

#[test]
fn parse_fg_colors_bright() {
    let input = "^fg(bright-green)I'm bright green text ^fg(bright-magenta)I am bright magenta";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Fg(term::color::BRIGHT_GREEN),
                  Literal("I'm bright green text ".to_string()),
                  Fg(term::color::BRIGHT_MAGENTA),
                  Literal("I am bright magenta".to_string())]))
}

#[test]
fn parse_fg_bg_colors() {
    let input = "^fg(bright-green)^bg(blue)I'm bright green text ^bg(bright-black)^fg(bright-magenta)I am bright magenta";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Fg(term::color::BRIGHT_GREEN),
                  Bg(term::color::BLUE),
                  Literal("I'm bright green text ".to_string()),
                  Bg(term::color::BRIGHT_BLACK),
                  Fg(term::color::BRIGHT_MAGENTA),
                  Literal("I am bright magenta".to_string())]))
}

#[test]
fn parse_fg_bg_bold_colors() {
    let input = "^fg(bright-green)^bg(blue)^bold()I'm bold bright green text";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Fg(term::color::BRIGHT_GREEN),
                  Bg(term::color::BLUE),
                  Bold,
                  Literal("I'm bold bright green text".to_string()),
                  ]))
}

#[test]
fn parse_dim() {
    let input = "^dim()I'm just dim text";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Dim,
                  Literal("I'm just dim text".to_string()),
                  ]))
}

#[test]
fn parse_reset() {
    let input = "^fg(red)I'm just dim text^reset()";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Fg(term::color::RED),
                  Literal("I'm just dim text".to_string()),
                  Reset,
                  ]))
}

#[test]
fn parse_italic() {
    let input = "^italic(true)I'm just dim text^italic(false)";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Italic(true),
                  Literal("I'm just dim text".to_string()),
                  Italic(false),
                  ]))
}

#[test]
fn parse_underline() {
    let input = "^underline(true)I'm underlined text^underline(false)";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Underline(true),
                  Literal("I'm underlined text".to_string()),
                  Underline(false),
                  ]))
}

#[test]
fn parse_blink() {
    let input = "^blink()I'm blinking text^reset()";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Blink,
                  Literal("I'm blinking text".to_string()),
                  Reset,
                  ]))
}

#[test]
fn parse_standout() {
    let input = "^standout(true)I'm standing out text^standout(false)";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Standout(true),
                  Literal("I'm standing out text".to_string()),
                  Standout(false),
                  ]))
}

#[test]
fn parse_reverse() {
    let input = "^reverse()I'm reversed text^reset()";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Reverse,
                  Literal("I'm reversed text".to_string()),
                  Reset,
                  ]))
}

#[test]
fn parse_secure() {
    let input = "^secure()I'm secure text^reset()";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Secure,
                  Literal("I'm secure text".to_string()),
                  Reset,
                  ]))
}
