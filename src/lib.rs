use State::{Beginning, Tag, Inside};
use Token::{Fg, Literal};

#[deriving(Show)]
enum State {
    Beginning,
    Tag,
    Inside,
}

#[deriving(Show, PartialEq, Eq)]
enum Token {
    Fg(String),
    Literal(String),
}

fn parse(s: String) -> Result<Vec<Token>, String> {
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
                            'a'...'z' => {
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
                                    tokens.push(Fg(current.clone()));
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

pub fn prepare(s: String) -> Result<String, String> {
    Ok("".into_string())
}


#[test]
fn parse_fg_two_colors() {
    let input = "^fg(red)I'm red text ^fg(blue)I am blue";
    assert!(parse(input.into_string())
         == Ok(
             vec![Literal("".into_string()),
              Fg("red".into_string()),
              Literal("I'm red text ".into_string()),
              Fg("blue".into_string()),
              Literal("I am blue".into_string())]))

}

// red   '\033[0;31m'
// reset '\033[0m'
#[test]
fn fg_two_colors() {
    let input = "^fg(red)I'm red text ^fg(blue)I am blue";
    assert!(prepare(input.into_string()).unwrap().as_slice()
         == "\033[0;31mI'm red text\033[0;34mI am blue\033[0m")
}
