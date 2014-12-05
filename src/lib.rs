use State::{Beginning, Opening, Inside, Closing, Literal};

enum State {
    Beginning,
    Opening,
    Inside,
    Closing,
    Literal,
}

pub fn prepare(s: String) -> Result<String, String> {
    let mut r;
    let mut state = Beginning;
    let mut current = String::new();
    let mut tokens = vec![];
    for i in s.chars() {
        match state {
            Beginning => match i {
                '^' => {
                    state = Opening;
                    tokens.push(current);
                },
                _   => {
                    state = Literal;
                    current.grow(1, i);
                },
            },
            Opening => match i {
                '(' => {
                    state = Inside;
                },
                  _ => return Err(format!("Expected '(', found {}", i)),
                },
            Inside => match i {
                ')' => {
                    state = Closing;
                    tokens.push(current);
                }
                  _ => {
                    state = Inside;
                    current.grow(1, i);
                },
            },
        }
    }
}


// red   '\033[0;31m'
// reset '\033[0m'
#[test]
fn fg_two_colors() {
    let input = "^fg(red)I'm red text ^fg(blue)I am blue";
    assert!(prepare(input.into_string()).unwrap().as_slice()
         == "\033[0;31mI'm red text\033[0;34mI am blue\033[0m")
}
