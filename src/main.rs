use ansi_term::Color::*;

const FERRIS: &'static str = r#"
█ █         █ █
▀█  ▄█████▄  █▀
 ▀▄███▀█▀███▄▀ 
 ▄▀███▀▀▀███▀▄ 
 █ ▄▀▀▀▀▀▀▀▄ █ 
"#;

fn main() {
    let lines = FERRIS.lines().map(|s| s.to_string()).collect::<Vec<String>>();
    let colors = &[Black, Red, Green, Yellow, Blue, Purple, Cyan, White];

    for line in &lines {
        for color in colors {
            print!("{} ", color.paint(line));
        }
        println!("");
    }

    for line in &lines {
        for color in colors {
            print!("{} ", color.bold().paint(line));
        }
        println!("");
    }
}