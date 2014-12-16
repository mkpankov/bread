# bread [![Build Status](https://travis-ci.org/mkpankov/bread.svg?branch=master)](https://travis-ci.org/mkpankov/bread)

Text formatting library for terminal output, with embedded formatting tokens.

We're going to use formatting syntax of Dzen (https://github.com/robm/dzen). Some examples:

* `^fg(red)I'm red text ^fg(blue)I am blue`
* `^bg(red)The ^fg(bright-black)text to ^bg(blue)^fg(cyan)colorize`

For full example of working program, see `src/test.rs`. You can run it by cloning the repo and doing `cargo run`.

To use in your project, add

```
[dependencies.bread]

git = "https://github.com/mkpankov/bread.git"

```

to `Cargo.toml` and you should be good to go.

## List of currently supported colors

* black
* blue
* bright-black
* bright-blue
* bright-cyan
* bright-green
* bright-magenta
* bright-red
* bright-white
* bright-yellow
* cyan
* green
* magenta
* red
* white
* yellow

## Demo

[![Demo](https://raw.githubusercontent.com/mkpankov/bread/master/show.gif)](https://github.com/mkpankov/bread)
