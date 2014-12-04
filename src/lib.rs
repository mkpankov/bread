use List::{Next, End};
use Expression::{Literal, Foreground};

#[deriving (Show)]
enum Color {
    Red,
    Blue,
}

#[deriving (Show)]
enum Expression {
    Literal(String),
    Foreground(Color),
}

#[deriving (Show)]
enum List {
    Next(Expression, Box<List>),
    End
}

#[deriving (Show)]
struct Parser {
    sentence: List,
    current: Option<Expression>,
}

fn prepare_folder(a: &mut Parser, b: char) -> Option<Parser> {
    let r = match b {
        _ => match *a {
            Parser { ref sentence, current: ref maybe_current } => {
                match maybe_current {
                    &None =>
                        Parser { sentence: *sentence,
                                 current: Some(Literal(String::from_char(1, b))) },
                    &Some(current) =>
                        match current {
                            Literal(string) =>
                                Parser { sentence: *sentence,
                                         current: Some(Literal([string, String::from_char(1, b)].concat())) },
                            Foreground(color) =>
                                Parser { sentence:
                                         match sentence {
                                             &Next(h, e) => Next(h, box Next(current, e)),
                                             &End => Next(current, box End),
                                         },
                                         current: Some(Literal(String::from_char(1, b))) }
                        }
                }
            }
        }
    };
    Some(r)
}

pub fn prepare(s: String) -> String {
    let mut r;
    for i in s.chars().scan(Parser { sentence: End, current: None},
                            prepare_folder) {
        println!("{}", i);
        r = i;
    }
    format!("{}", r)
}


// red   '\033[0;31m'
// reset '\033[0m'
#[test]
fn fg_two_colors() {
    let input = "^fg(red)I'm red text ^fg(blue)I am blue";
    assert!(prepare(input.into_string()).as_slice()
         == "\033[0;31mI'm red text\033[0;34mI am blue\033[0m")
}
