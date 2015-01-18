extern crate term;

use State::{Beginning, Tag, Inside, InsideColor, InsideBool};
use Token::{Attribute,
            Reset,
            Literal,
            Partial,
};
use term::{StdTerminal, Attr};
pub use term::Attr::{
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Standout,
    Reverse,
    Secure,
    ForegroundColor,
    BackgroundColor,
};
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
    Attribute(Attr),
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
                    match try!(get_token_state(&*current)) {
                        (token, s) => {
                            tokens.push(token);
                            state = s;
                        }
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
                                    Partial(PT::Italic) => Attribute(Italic(value)),
                                    Partial(PT::Underline) => Attribute(Underline(value)),
                                    Partial(PT::Standout) => Attribute(Standout(value)),
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
                        let color = try!(get_color_by_name(&*current));
                        let maybe_last = tokens.pop();
                        tokens.push(match maybe_last {
                            None => return Err(format!("Expected a tag token in array, found {:?}", maybe_last)),
                            Some(token) => {
                                match token {
                                    Partial(PT::Fg) => Attribute(ForegroundColor(color)),
                                    Partial(PT::Bg) => Attribute(BackgroundColor(color)),
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

fn get_token_state(current: &str) -> Result<(Token, State), String> {
    match &*current {
        "fg" => {
            Ok((Partial(PT::Fg), InsideColor))
        }
        "bg" => {
            Ok((Partial(PT::Bg), InsideColor))
        }
        "bold" => {
            Ok((Attribute(Bold), Inside))
        }
        "dim" => {
            Ok((Attribute(Dim), Inside))
        }
        "italic" => {
            Ok((Partial(PT::Italic), InsideBool))
        }
        "underline" => {
            Ok((Partial(PT::Underline), InsideBool))
        }
        "blink" => {
            Ok((Attribute(Blink), Inside))
        }
        "standout" => {
            Ok((Partial(PT::Standout), InsideBool))
        }
        "reverse" => {
            Ok((Attribute(Reverse), Inside))
        }
        "secure" => {
            Ok((Attribute(Secure), Inside))
        }
        "reset" => {
            Ok((Reset, Inside))
        }
        _ => {
            Err(format!("Expected a tag, found {}", current))
        }
    }
}

fn get_color_by_name(color: &str) -> Result<Color, String> {
    match &*color {
        "black" => Ok(term::color::BLACK),
        "blue" => Ok(term::color::BLUE),
        "bright-black" => Ok(term::color::BRIGHT_BLACK),
        "bright-blue" => Ok(term::color::BRIGHT_BLUE),
        "bright-cyan" => Ok(term::color::BRIGHT_CYAN),
        "bright-green" => Ok(term::color::BRIGHT_GREEN),
        "bright-magenta" => Ok(term::color::BRIGHT_MAGENTA),
        "bright-red" => Ok(term::color::BRIGHT_RED),
        "bright-white" => Ok(term::color::BRIGHT_WHITE),
        "bright-yellow" => Ok(term::color::BRIGHT_YELLOW),
        "cyan" => Ok(term::color::CYAN),
        "green" => Ok(term::color::GREEN),
        "magenta" => Ok(term::color::MAGENTA),
        "red" => Ok(term::color::RED),
        "white" => Ok(term::color::WHITE),
        "yellow" => Ok(term::color::YELLOW),

        _ => return Err(format!("Expected color name, found {}", color)),
    }
}

pub fn render(trm: &mut FullTerminal, tokens: &[Token]) {
    for t in tokens.iter() {
        match *t {
            Literal(ref string) => write!(trm, "{}", string).unwrap(),
            Attribute(value) => {
                trm.attr(value).unwrap();
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
             vec![Attribute(ForegroundColor(term::color::RED)),
                  Literal("I'm red text ".to_string()),
                  Attribute(ForegroundColor(term::color::BLUE)),
                  Literal("I am blue".to_string())]))
}

#[test]
fn parse_fg_colors_bright() {
    let input = "^fg(bright-green)I'm bright green text ^fg(bright-magenta)I am bright magenta";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Attribute(ForegroundColor(term::color::BRIGHT_GREEN)),
                  Literal("I'm bright green text ".to_string()),
                  Attribute(ForegroundColor(term::color::BRIGHT_MAGENTA)),
                  Literal("I am bright magenta".to_string())]))
}

#[test]
fn parse_fg_bg_colors() {
    let input = "^fg(bright-green)^bg(blue)I'm bright green text ^bg(bright-black)^fg(bright-magenta)I am bright magenta";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Attribute(ForegroundColor(term::color::BRIGHT_GREEN)),
                  Attribute(BackgroundColor(term::color::BLUE)),
                  Literal("I'm bright green text ".to_string()),
                  Attribute(BackgroundColor(term::color::BRIGHT_BLACK)),
                  Attribute(ForegroundColor(term::color::BRIGHT_MAGENTA)),
                  Literal("I am bright magenta".to_string())]))
}

#[test]
fn parse_fg_bg_bold_colors() {
    let input = "^fg(bright-green)^bg(blue)^bold()I'm bold bright green text";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Attribute(ForegroundColor(term::color::BRIGHT_GREEN)),
                  Attribute(BackgroundColor(term::color::BLUE)),
                  Attribute(Bold),
                  Literal("I'm bold bright green text".to_string()),
                  ]))
}

#[test]
fn parse_dim() {
    let input = "^dim()I'm just dim text";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Attribute(Dim),
                  Literal("I'm just dim text".to_string()),
                  ]))
}

#[test]
fn parse_reset() {
    let input = "^fg(red)I'm just dim text^reset()";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Attribute(ForegroundColor(term::color::RED)),
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
             vec![Attribute(Italic(true)),
                  Literal("I'm just dim text".to_string()),
                  Attribute(Italic(false)),
                  ]))
}

#[test]
fn parse_underline() {
    let input = "^underline(true)I'm underlined text^underline(false)";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Attribute(Underline(true)),
                  Literal("I'm underlined text".to_string()),
                  Attribute(Underline(false)),
                  ]))
}

#[test]
fn parse_blink() {
    let input = "^blink()I'm blinking text^reset()";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Attribute(Blink),
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
             vec![Attribute(Standout(true)),
                  Literal("I'm standing out text".to_string()),
                  Attribute(Standout(false)),
                  ]))
}

#[test]
fn parse_reverse() {
    let input = "^reverse()I'm reversed text^reset()";
    println!("{:?}", parse(input));
    assert!(parse(input)
         == Ok(
             vec![Attribute(Reverse),
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
             vec![Attribute(Secure),
                  Literal("I'm secure text".to_string()),
                  Reset,
                  ]))
}
