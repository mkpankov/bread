fn prepare(s: &str) -> String {
    s.to_string()
}


// red   '\033[0;31m'
// reset '\033[0m'
#[test]
fn fg_two_colors() {
    let input = "^fg(red)I'm red text ^fg(blue)I am blue";
    assert!(prepare(input).as_slice() == "\033[0;31mI'm red text\033[0;34mI am blue\033[0m")
}
