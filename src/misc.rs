use crate::Options;
use console::{Key, Term};
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
pub fn convert(c: &char) -> char
{
    let valid_chars = [
        '+', '^', '(', ')', '.', '=', ',', '#', '|', '&', '!', '%', '_', '<', '>', ' ', '[', ']',
        '{', '}', '√', '∛', '⁻', 'ⁱ', '`', '±',
    ];
    match c
    {
        c if c.is_alphanumeric() || valid_chars.contains(c) => *c,
        '∗' | '∙' | '·' | '⋅' | '*' => '*',
        '∕' | '⁄' | '/' => '/',
        '−' | '-' => '-',
        _ => '\0',
    }
}
pub fn read_single_char() -> char
{
    let term = Term::stdout();
    let key = term.read_key().unwrap();
    match key
    {
        Key::Char(c) => convert(&c),
        Key::Enter => '\n',
        Key::Backspace => '\x08',
        Key::ArrowLeft => '\x1B',
        Key::ArrowRight => '\x1C',
        Key::ArrowUp => '\x1D',
        Key::ArrowDown => '\x1E',
        Key::Escape => '\x1A',
        _ => '\0',
    }
}
pub fn write(input: &str, file: &mut File, lines: &Vec<String>)
{
    if lines.is_empty() || lines[lines.len() - 1] != *input
    {
        file.write_all(input.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}
pub fn clear(input: &[char], start: usize, end: usize, options: Options)
{
    print!(
        "\x1B[0J\x1B[2K\x1B[1G{}{}\x1b[0m",
        if options.prompt
        {
            if options.color
            {
                "\x1b[94m> \x1b[96m"
            }
            else
            {
                "> "
            }
        }
        else if options.color
        {
            "\x1b[96m"
        }
        else
        {
            ""
        },
        &input[start..end].iter().collect::<String>()
    );
}
pub fn handle_err(err: &str, input: &[char], options: Options, start: usize, end: usize)
{
    print!(
        "\x1B[0J\x1B[2K\x1B[1G\n{}\x1b[A\x1B[2K\x1B[1G{}{}{}",
        err,
        if options.prompt
        {
            if options.color
            {
                "\x1b[94m> \x1b[96m"
            }
            else
            {
                "> "
            }
        }
        else if options.color
        {
            "\x1b[96m"
        }
        else
        {
            ""
        },
        &input[start..end].iter().collect::<String>(),
        if options.color { "\x1b[0m" } else { "" },
    );
}