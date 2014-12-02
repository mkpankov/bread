pub fn prepare(s: String) -> String {
    s.chars().fold("".to_string(), |a, b|
                   match b {
                       '.' => [a.as_slice(), ":"].concat(),
                       _ => [a, String::from_char(1, b)].concat(),
                   }
                   )
}


// red   '\033[0;31m'
// reset '\033[0m'
#[test]
fn fg_two_colors() {
    let input = "^fg(red)I'm red text ^fg(blue)I am blue";
    assert!(prepare(input.into_string()).as_slice() == "\033[0;31mI'm red text\033[0;34mI am blue\033[0m")
}
