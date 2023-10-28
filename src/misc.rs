use crate::{Colors, Options};
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
pub fn get_terminal_width() -> usize
{
    unsafe {
        let mut size: winsize = std::mem::zeroed();
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) == 0 && size.ws_col != 0
        {
            size.ws_col as usize
        }
        else
        {
            80
        }
    }
}
#[cfg(not(unix))]
pub fn get_terminal_width() -> usize
{
    if let Some((width, _)) = dimensions()
    {
        width
    }
    else
    {
        80
    }
}
pub fn digraph() -> char
{
    match read_single_char()
    {
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
        'K' => 'Κ',
        'l' => 'λ',
        'L' => 'Λ',
        'm' => 'μ',
        'M' => 'Μ',
        'n' => 'ν',
        'N' => 'Ν',
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
        '`' => 'ⁱ',
        '=' => '±',
        '_' => '∞',
        '\n' => '\n',
        '\x08' => '\x08',
        '\x7F' => '\x7F',
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
pub fn convert(c: &char) -> char
{
    let valid_chars = [
        '+', '^', '(', ')', '.', '=', ',', '#', '|', '&', '!', '%', '_', '<', '>', ' ', '[', ']',
        '{', '}', '√', '∛', '⁻', 'ⁱ', '`', '±', '∞', ';',
    ];
    match c
    {
        c if c.is_alphanumeric() || valid_chars.contains(c) => *c,
        '∗' | '∙' | '·' | '⋅' | '*' | '×' => '*',
        '∕' | '⁄' | '/' => '/',
        '−' | '-' => '-',
        _ => '\0',
    }
}
pub fn read_single_char() -> char
{
    terminal::enable_raw_mode().unwrap();
    let result =
        match match read()
        {
            Ok(c) => c,
            _ => return '\0',
        }
        {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers)
            {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => '\x14',
                (KeyCode::Home, KeyModifiers::NONE)
                | (KeyCode::Char('a'), KeyModifiers::CONTROL) => '\x10',
                (KeyCode::End, KeyModifiers::NONE)
                | (KeyCode::Char('e'), KeyModifiers::CONTROL) => '\x11',
                (KeyCode::Left, KeyModifiers::CONTROL)
                | (KeyCode::Char('b'), KeyModifiers::ALT) => '\x12',
                (KeyCode::Right, KeyModifiers::CONTROL)
                | (KeyCode::Char('f'), KeyModifiers::ALT) => '\x13',
                (KeyCode::Char('l'), KeyModifiers::CONTROL) => '\x15',
                (KeyCode::Char('t'), KeyModifiers::CONTROL) => '\x16',
                (KeyCode::Char('u'), KeyModifiers::CONTROL) => '\x18',
                (KeyCode::Char('k'), KeyModifiers::CONTROL) => '\x19',
                (KeyCode::Char('y'), KeyModifiers::CONTROL) => '\x17',
                (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => convert(&c),
                (KeyCode::Esc, _) => digraph(),
                (KeyCode::Enter, KeyModifiers::NONE) => '\n',
                (KeyCode::Backspace, KeyModifiers::NONE) => '\x08',
                (KeyCode::Delete, KeyModifiers::NONE) => '\x7F',
                (KeyCode::Left, KeyModifiers::NONE) => '\x1B',
                (KeyCode::Right, KeyModifiers::NONE) => '\x1C',
                (KeyCode::Up, KeyModifiers::NONE) => '\x1D',
                (KeyCode::Down, KeyModifiers::NONE) => '\x1E',
                _ => '\0',
            },
            _ => '\0',
        };
    terminal::disable_raw_mode().unwrap();
    if result == '\x14'
    {
        print!("\x1b[G\x1b[J");
        std::process::exit(130);
    }
    result
}
pub fn no_col(input: &str, color: bool) -> String
{
    if color
    {
        let mut skip = false;
        let mut output = String::new();
        for c in input.chars()
        {
            if skip
            {
                if c == 'm'
                {
                    skip = false
                }
            }
            else if c == '\x1b'
            {
                skip = true
            }
            else
            {
                output.push(c)
            }
        }
        output
    }
    else
    {
        input.to_string()
    }
}
pub fn write(input: &str, file: &mut File, lines: &mut Vec<String>)
{
    if !lines.is_empty() && lines.last().unwrap() != input
    {
        lines.push(input.to_string());
        file.write_all(input.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}
pub fn clearln(input: &[char], start: usize, end: usize, options: Options, colors: &Colors)
{
    print!(
        "\x1b[G\x1b[K{}{}{}",
        prompt(options, colors),
        to_output(&input[start..end], options.color, colors),
        if options.color { "\x1b[0m" } else { "" }
    );
}
pub fn clear(input: &[char], start: usize, end: usize, options: Options, colors: &Colors)
{
    print!(
        "\x1b[G\x1b[J{}{}{}",
        prompt(options, colors),
        to_output(&input[start..end], options.color, colors),
        if options.color { "\x1b[0m" } else { "" }
    );
}
pub fn to_output(input: &[char], color: bool, colors: &Colors) -> String
{
    if color
    {
        let mut count: isize = 0;
        let mut output = String::new();
        let mut abs = false;
        for c in input
        {
            match c
            {
                '|' =>
                {
                    if abs
                    {
                        count -= 1;
                        output.push_str(&format!(
                            "{}|{}",
                            colors.brackets[count as usize % colors.brackets.len()],
                            colors.text
                        ))
                    }
                    else
                    {
                        output.push_str(&format!(
                            "{}|{}",
                            colors.brackets[count as usize % colors.brackets.len()],
                            colors.text
                        ));
                        count += 1
                    }
                    abs = !abs
                }
                '(' =>
                {
                    output.push_str(&format!(
                        "{}({}",
                        colors.brackets[count as usize % colors.brackets.len()],
                        colors.text
                    ));
                    count += 1
                }
                ')' =>
                {
                    count -= 1;
                    output.push_str(&format!(
                        "{}){}",
                        colors.brackets[count as usize % colors.brackets.len()],
                        colors.text
                    ))
                }
                _ => output.push(*c),
            }
        }
        output
    }
    else
    {
        input.iter().collect::<String>()
    }
}
pub fn handle_err(
    err: &str,
    input: &[char],
    options: Options,
    colors: &Colors,
    start: usize,
    end: usize,
)
{
    print!(
        "\x1b[J\n{}{}\x1b[A\x1b[G\x1b[K{}{}{}",
        err,
        "\x1b[A".repeat(err.len().div_ceil(get_terminal_width()) - 1),
        prompt(options, colors),
        to_output(&input[start..end], options.color, colors),
        if options.color { "\x1b[0m" } else { "" },
    );
}
pub fn prompt(options: Options, colors: &Colors) -> String
{
    if options.prompt
    {
        if options.color
        {
            format!("{}> {}", colors.prompt, colors.text)
        }
        else
        {
            "> ".to_string()
        }
    }
    else if options.color
    {
        colors.text.to_string()
    }
    else
    {
        "".to_string()
    }
}