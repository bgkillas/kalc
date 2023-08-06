use crate::Options;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal,
};
#[cfg(unix)]
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::{fs::File, io::Write};
#[cfg(not(unix))]
use term_size::dimensions;
#[cfg(unix)]
pub fn get_terminal_width() -> usize {
    unsafe {
        let mut size: winsize = std::mem::zeroed();
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) == 0 && size.ws_col != 0 {
            size.ws_col as usize
        } else {
            80
        }
    }
}
#[cfg(not(unix))]
pub fn get_terminal_width() -> usize {
    if let Some((width, _)) = dimensions() {
        width
    } else {
        80
    }
}
pub fn digraph() -> char {
    match read_single_char() {
        'a' => 'α',
        'A' => 'Α',
        'b' => 'β',
        'B' => 'Β',
        'c' => 'ξ',
        'C' => 'Ξ',
        'd' => 'Δ',
        'D' => 'δ',
        'e' => 'ε',
        'E' => 'Ε',
        'f' => 'φ',
        'F' => 'Φ',
        'g' => 'γ',
        'G' => 'Γ',
        'h' => 'η',
        'H' => 'Η',
        'i' => 'ι',
        'I' => 'Ι',
        'k' => 'κ',
        'Κ' => 'Κ',
        'l' => 'λ',
        'L' => 'Λ',
        'm' => 'μ',
        'M' => 'Μ',
        'n' => 'ν',
        'Ν' => 'Ν',
        'o' => 'ο',
        'O' => 'Ο',
        'p' => 'π',
        'P' => 'Π',
        'q' => 'θ',
        'Q' => 'Θ',
        'r' => 'ρ',
        'R' => 'Ρ',
        's' => 'σ',
        'S' => 'Σ',
        't' => 'τ',
        'T' => 'Τ',
        'u' => 'υ',
        'U' => 'Υ',
        'w' => 'ω',
        'W' => 'Ω',
        'y' => 'ψ',
        'Y' => 'Ψ',
        'x' => 'χ',
        'X' => 'Χ',
        'z' => 'ζ',
        'Z' => 'Ζ',
        '0' => '⁰',
        '9' => '⁹',
        '8' => '⁸',
        '7' => '⁷',
        '6' => '⁶',
        '5' => '⁵',
        '4' => '⁴',
        '3' => '³',
        '2' => '²',
        '1' => '¹',
        '-' => '⁻',
        ']' => 'ⁱ',
        '=' => '±',
        '\n' => '\n',
        '\x08' => '\x08',
        '\x1B' => '\x1B',
        '\x1C' => '\x1C',
        '\x1D' => '\x1D',
        '\x1E' => '\x1E',
        '\x1A' => '\x1A',
        '\x10' => '\x10',
        '\x11' => '\x11',
        '\x12' => '\x12',
        '\x13' => '\x13',
        _ => '\0',
    }
}
pub fn convert(c: &char) -> char {
    let valid_chars = [
        '+', '^', '(', ')', '.', '=', ',', '#', '|', '&', '!', '%', '_', '<', '>', ' ', '[', ']',
        '{', '}', '√', '∛', '⁻', 'ⁱ', '`', '±',
    ];
    match c {
        c if c.is_alphanumeric() || valid_chars.contains(c) => *c,
        '∗' | '∙' | '·' | '⋅' | '*' => '*',
        '∕' | '⁄' | '/' => '/',
        '−' | '-' => '-',
        _ => '\0',
    }
}
pub fn read_single_char() -> char {
    terminal::enable_raw_mode().unwrap();
    let result = match match read() {
        Ok(c) => c,
        Err(_) => return '\0',
    } {
        Event::Key(KeyEvent {
            code, modifiers, ..
        }) => match (code, modifiers) {
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => '\x14',
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => convert(&c),
            (KeyCode::Esc, _) => digraph(),
            (KeyCode::Enter, KeyModifiers::NONE) => '\n',
            (KeyCode::Backspace, KeyModifiers::NONE) => '\x08',
            (KeyCode::Left, KeyModifiers::NONE) => '\x1B',
            (KeyCode::Right, KeyModifiers::NONE) => '\x1C',
            (KeyCode::Left, KeyModifiers::ALT) => '\x12',
            (KeyCode::Right, KeyModifiers::ALT) => '\x13',
            (KeyCode::Up, KeyModifiers::NONE) => '\x1D',
            (KeyCode::Down, KeyModifiers::NONE) => '\x1E',
            (KeyCode::End, KeyModifiers::NONE) => '\x11',
            (KeyCode::Home, KeyModifiers::NONE) => '\x10',
            _ => '\0',
        },
        _ => '\0',
    };
    terminal::disable_raw_mode().unwrap();
    if result == '\x14' {
        println!();
        std::process::exit(130);
    }
    result
}
pub fn write(input: &str, file: &mut File, lines: &Vec<String>) {
    if lines.is_empty() || lines[lines.len() - 1] != *input {
        file.write_all(input.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}
pub fn clear(input: &[char], start: usize, end: usize, options: Options) {
    print!(
        "\x1B[0J\x1B[2K\x1B[1G{}{}\x1b[0m",
        if options.prompt {
            if options.color {
                "\x1b[94m> \x1b[96m"
            } else {
                "> "
            }
        } else if options.color {
            "\x1b[96m"
        } else {
            ""
        },
        &input[start..end].iter().collect::<String>()
    );
}
pub fn handle_err(err: &str, input: &[char], options: Options, start: usize, end: usize) {
    print!(
        "\x1B[0J\x1B[2K\x1B[1G\n{}{}\x1b[A\x1B[2K\x1B[1G{}{}{}",
        err,
        "\x1b[A".repeat((err.len() as f64 / get_terminal_width() as f64).ceil() as usize - 1),
        if options.prompt {
            if options.color {
                "\x1b[94m> \x1b[96m"
            } else {
                "> "
            }
        } else if options.color {
            "\x1b[96m"
        } else {
            ""
        },
        &input[start..end].iter().collect::<String>(),
        if options.color { "\x1b[0m" } else { "" },
    );
}
