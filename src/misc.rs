use crate::{
    complex::{
        NumStr,
        NumStr::{
            And, Comma, Converse, Conversion, Division, Equal, Exponent, Func, Greater,
            GreaterEqual, Implies, InternalMultiplication, LeftBracket, LeftCurlyBracket, Lesser,
            LesserEqual, Matrix, Minus, Modulo, Multiplication, Nand, NearEqual, Nor, Not,
            NotEqual, Num, Or, Plus, PlusMinus, Range, RightBracket, RightCurlyBracket, Root,
            ShiftLeft, ShiftRight, Tetration, Vector, Xor,
        },
    },
    functions::functions,
    math::do_math,
    print::{custom_units, get_output},
    Colors, Options, Variable,
};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
#[cfg(unix)]
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::{fs::File, io::Write};
#[cfg(not(unix))]
use term_size::dimensions;
#[cfg(unix)]
pub fn get_terminal_dimensions() -> (usize, usize)
{
    unsafe {
        let mut size: winsize = std::mem::zeroed();
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) == 0 && size.ws_col != 0
        {
            (size.ws_col as usize, size.ws_row as usize)
        }
        else
        {
            (80, 80)
        }
    }
}
#[cfg(unix)]
pub fn get_terminal_dimensions_pixel() -> (usize, usize)
{
    unsafe {
        let mut size: winsize = std::mem::zeroed();
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) == 0 && size.ws_col != 0
        {
            (size.ws_xpixel as usize, size.ws_ypixel as usize)
        }
        else
        {
            (80, 80)
        }
    }
}
#[cfg(not(unix))]
pub fn get_terminal_dimensions() -> (usize, usize)
{
    if let Some((width, height)) = dimensions()
    {
        (width, height)
    }
    else
    {
        (80, 80)
    }
}
pub fn digraph(char: Option<char>) -> char
{
    match if let Some(c) = char
    {
        c
    }
    else
    {
        read_single_char()
    }
    {
        ';' => '°',
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
        '+' => '±',
        '=' => '≈',
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
        '{', '}', '√', '∛', '⁻', 'ⁱ', '`', '±', '∞', ';', ':', '\'', '⌊', '⌈', '⌉', '⌋', '∫', '°',
        '$', '¢', '≈', '~', '¬',
    ];
    match c
    {
        c if c.is_alphanumeric() || valid_chars.contains(c) => *c,
        '∗' | '∙' | '·' | '⋅' | '*' | '×' => '*',
        '∕' | '⁄' | '/' | '÷' => '/',
        '−' | '-' => '-',
        _ => '\0',
    }
}
pub fn read_single_char() -> char
{
    match match read()
    {
        Ok(c) => c,
        _ => return '\0',
    }
    {
        Event::Key(KeyEvent {
            code,
            modifiers,
            kind,
            ..
        }) =>
        {
            if kind == KeyEventKind::Press
            {
                match (code, modifiers)
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
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => '\x06',
                    (KeyCode::Char('p'), KeyModifiers::CONTROL) => '\x05',
                    (KeyCode::Char('n'), KeyModifiers::CONTROL) => '\x04',
                    (KeyCode::Char('d'), KeyModifiers::ALT) => '\x0C',
                    (KeyCode::Char('w'), KeyModifiers::CONTROL) => '\x0D',
                    (KeyCode::Char('x'), KeyModifiers::CONTROL) => '\x0E',
                    (KeyCode::Char('t'), KeyModifiers::ALT) => '\x0F',
                    (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => convert(&c),
                    (KeyCode::Char(c), KeyModifiers::ALT) => digraph(Some(c)),
                    (KeyCode::Esc, _) => digraph(None),
                    (KeyCode::Enter, KeyModifiers::NONE) => '\n',
                    (KeyCode::Enter, KeyModifiers::ALT) => '\x09',
                    (KeyCode::Char('h'), KeyModifiers::CONTROL) => '\x03',
                    (KeyCode::Backspace, KeyModifiers::NONE) => '\x08',
                    (KeyCode::Delete, KeyModifiers::NONE) => '\x7F',
                    (KeyCode::Left, KeyModifiers::NONE)
                    | (KeyCode::Char('b'), KeyModifiers::CONTROL) => '\x1B',
                    (KeyCode::Right, KeyModifiers::NONE)
                    | (KeyCode::Char('f'), KeyModifiers::CONTROL) => '\x1C',
                    (KeyCode::Up, KeyModifiers::NONE) => '\x1D',
                    (KeyCode::Down, KeyModifiers::NONE) => '\x1E',
                    (KeyCode::Tab, KeyModifiers::NONE) => '\x1F',
                    _ => '\0',
                }
            }
            else
            {
                '\0'
            }
        }
        _ => '\0',
    }
}
pub fn end_word(c: char) -> bool
{
    matches!(
        c,
        '(' | '{'
            | '['
            | ')'
            | '}'
            | ']'
            | '+'
            | '-'
            | '*'
            | '/'
            | '^'
            | '<'
            | '='
            | '>'
            | '|'
            | '&'
            | '!'
            | '±'
            | '%'
    )
}
pub fn no_col(input: &str, color: bool) -> Vec<char>
{
    if color
    {
        let mut skip = false;
        let mut output = String::new();
        for c in input.chars()
        {
            if skip
            {
                if c == 'm' || c == 'G' || c == 'K'
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
        output.chars().collect::<Vec<char>>()
    }
    else
    {
        input.chars().collect::<Vec<char>>()
    }
}
pub fn no_col_len(input: &str, color: bool) -> usize
{
    if color
    {
        let mut skip = false;
        let mut count = 0;
        for c in input.chars()
        {
            if skip
            {
                if c == 'm' || c == 'G' || c == 'K'
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
                count += 1
            }
        }
        count
    }
    else
    {
        input.len()
    }
}
pub fn write(mut input: String, file: &mut File, lines: &mut Vec<String>, slow: bool, last: String)
{
    if last != input && !input.replace(' ', "").is_empty() && !input.starts_with('#')
    {
        if slow
        {
            input.push('\t');
        }
        lines.push(input.clone());
        writeln!(file, "{}", input).unwrap();
    }
}
pub fn clearln(
    input: &[char],
    vars: &[Variable],
    start: usize,
    end: usize,
    options: Options,
    colors: &Colors,
)
{
    print!(
        "\x1b[G{}{}\x1b[K{}",
        prompt(options, colors),
        to_output(
            &input[start..end],
            vars,
            options.color == crate::Auto::True,
            colors
        ),
        if options.color == crate::Auto::True
        {
            "\x1b[0m"
        }
        else
        {
            ""
        }
    );
}
pub fn clear(
    input: &[char],
    vars: &[Variable],
    start: usize,
    end: usize,
    options: Options,
    colors: &Colors,
)
{
    print!(
        "\x1b[G{}{}\x1b[J{}",
        prompt(options, colors),
        to_output(
            &input[start..end],
            vars,
            options.color == crate::Auto::True,
            colors
        ),
        if options.color == crate::Auto::True
        {
            "\x1b[0m"
        }
        else
        {
            ""
        }
    );
}
pub fn to_output(input: &[char], vars: &[Variable], color: bool, colors: &Colors) -> String
{
    if color
    {
        let mut count: isize = (input
            .iter()
            .filter(|a| matches!(a, ')' | '}' | ']'))
            .count() as isize
            - input
                .iter()
                .filter(|a| matches!(a, '(' | '{' | '['))
                .count() as isize)
            .max(0);
        let mut output = String::new();
        let mut abs = Vec::new();
        let mut i = 0;
        let mut ignore = false;
        while i < input.len()
        {
            let c = input[i];
            match c
            {
                '#' =>
                {
                    count = 0;
                    output.push(c)
                }
                '\x1b' =>
                {
                    ignore = true;
                    output.push(c)
                }
                '[' if ignore => output.push(c),
                '|' =>
                {
                    if !abs.is_empty()
                        && abs[0] == count
                        && match input[..i].iter().rev().position(|c| !c.is_alphabetic())
                        {
                            Some(n) =>
                            {
                                let s = input[i - n..i].iter().collect::<String>();
                                let sb = &(s.clone() + "(");
                                !(functions().contains(s.as_str())
                                    || vars
                                        .iter()
                                        .any(|c| c.name.iter().collect::<String>().starts_with(sb)))
                            }
                            _ => true,
                        }
                    {
                        count -= 1;
                        abs.remove(0);
                        output.push_str(&format!(
                            "{}|{}",
                            colors.brackets[count as usize % colors.brackets.len()],
                            colors.text
                        ))
                    }
                    else if i + 1 == input.len() || input[i + 1] != '|'
                    {
                        output.push_str(&format!(
                            "{}|{}",
                            colors.brackets[count as usize % colors.brackets.len()],
                            colors.text
                        ));
                        count += 1;
                        abs.insert(0, count);
                    }
                    else
                    {
                        i += 1;
                        output.push_str("||")
                    }
                }
                '(' | '{' | '[' =>
                {
                    output.push_str(&format!(
                        "{}{}{}",
                        colors.brackets[count as usize % colors.brackets.len()],
                        c,
                        colors.text
                    ));
                    count += 1
                }
                ')' | '}' | ']' =>
                {
                    count -= 1;
                    output.push_str(&format!(
                        "{}{}{}",
                        colors.brackets[count as usize % colors.brackets.len()],
                        c,
                        colors.text
                    ))
                }
                '@' =>
                {}
                _ => output.push(c),
            }
            i += 1;
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
    vars: &[Variable],
    input: &[char],
    options: Options,
    colors: &Colors,
    start: usize,
    end: usize,
)
{
    let num = err.len().div_ceil(get_terminal_dimensions().0) - 1;
    print!(
        "\x1b[J\x1b[G\n{}{}\x1b[G\x1b[A\x1b[K{}{}{}",
        err,
        if num == 0
        {
            String::new()
        }
        else
        {
            format!("\x1b[{}A", num)
        },
        prompt(options, colors),
        to_output(
            &input[start..end],
            vars,
            options.color == crate::Auto::True,
            colors
        ),
        if options.color == crate::Auto::True
        {
            "\x1b[0m"
        }
        else
        {
            ""
        },
    );
}
pub fn prompt(options: Options, colors: &Colors) -> String
{
    if !options.interactive
    {
        String::new()
    }
    else if options.prompt
    {
        if options.color == crate::Auto::True
        {
            format!("{}>{} ", colors.prompt, colors.text)
        }
        else
        {
            "> ".to_string()
        }
    }
    else if options.color == crate::Auto::True
    {
        colors.text.to_string()
    }
    else
    {
        String::new()
    }
}
pub fn place_funcvarxy(
    mut funcvar: Vec<(String, Vec<NumStr>)>,
    num: NumStr,
) -> Vec<(String, Vec<NumStr>)>
{
    for i in funcvar.iter_mut()
    {
        if !i.0.contains('(')
        {
            let mut sum: Vec<(usize, String)> = Vec::new();
            let mut bracket = 0;
            let mut j = 0;
            while i.1.len() > j
            {
                match &i.1[j]
                {
                    LeftBracket => bracket += 1,
                    RightBracket => bracket -= 1,
                    Comma if !sum.is_empty() && sum[0].0 == bracket =>
                    {
                        sum.remove(0);
                    }
                    Func(s) =>
                    {
                        if matches!(s.as_str(), "x" | "y") && !sum.iter().any(|a| a.1 == *s)
                        {
                            i.1[j] = num.clone();
                        }
                        else
                        {
                            match s.as_str()
                            {
                                "sum" | "summation" | "prod" | "product" | "Σ" | "Π" | "vec"
                                | "mat" | "D" | "integrate" | "arclength" | "∫" | "area"
                                | "surfacearea" | "sarea" | "solve" | "length" | "slope"
                                | "lim" | "set" | "limit" | "iter" | "extrema"
                                    if j + 2 < i.1.len()
                                        && if let Func(s) = &i.1[j + 2]
                                        {
                                            matches!(s.as_str(), "x" | "y")
                                        }
                                        else
                                        {
                                            false
                                        } =>
                                {
                                    bracket += 1;
                                    j += 3;
                                    sum.push((
                                        bracket,
                                        if let Func(s) = &i.1[j - 1]
                                        {
                                            s.to_string()
                                        }
                                        else
                                        {
                                            String::new()
                                        },
                                    ))
                                }
                                "surfacearea" | "sarea"
                                    if j + 4 < i.1.len()
                                        && if let Func(s) = &i.1[j + 4]
                                        {
                                            matches!(s.as_str(), "x" | "y")
                                        }
                                        else
                                        {
                                            false
                                        } =>
                                {
                                    bracket += 1;
                                    j += 5;
                                    sum.push((
                                        bracket,
                                        if let Func(s) = &i.1[j - 1]
                                        {
                                            s.to_string()
                                        }
                                        else
                                        {
                                            String::new()
                                        },
                                    ))
                                }
                                _ =>
                                {}
                            }
                        }
                    }
                    _ =>
                    {}
                }
                j += 1;
            }
        }
    }
    funcvar
}
pub fn place_varxy(mut func: Vec<NumStr>, num: NumStr) -> Vec<NumStr>
{
    let mut sum: Vec<(usize, String)> = Vec::new();
    let mut bracket = 0;
    let mut i = 0;
    while func.len() > i
    {
        match &func[i]
        {
            LeftBracket => bracket += 1,
            RightBracket => bracket -= 1,
            Comma if !sum.is_empty() && sum[0].0 == bracket =>
            {
                sum.remove(0);
            }
            Func(s) =>
            {
                if matches!(s.as_str(), "x" | "y") && !sum.iter().any(|a| a.1 == *s)
                {
                    func[i] = num.clone();
                }
                else
                {
                    match s.as_str()
                    {
                        "sum" | "summation" | "prod" | "product" | "Σ" | "Π" | "vec" | "mat"
                        | "D" | "integrate" | "arclength" | "∫" | "area" | "surfacearea"
                        | "sarea" | "solve" | "length" | "slope" | "lim" | "limit" | "set"
                        | "iter" | "extrema"
                            if i + 2 < func.len()
                                && if let Func(s) = &func[i + 2]
                                {
                                    matches!(s.as_str(), "x" | "y")
                                }
                                else
                                {
                                    false
                                } =>
                        {
                            bracket += 1;
                            i += 3;
                            sum.push((
                                bracket,
                                if let Func(s) = &func[i - 1]
                                {
                                    s.to_string()
                                }
                                else
                                {
                                    String::new()
                                },
                            ))
                        }
                        "surfacearea" | "sarea"
                            if i + 4 < func.len()
                                && if let Func(s) = &func[i + 4]
                                {
                                    matches!(s.as_str(), "x" | "y")
                                }
                                else
                                {
                                    false
                                } =>
                        {
                            bracket += 1;
                            i += 5;
                            sum.push((
                                bracket,
                                if let Func(s) = &func[i - 1]
                                {
                                    s.to_string()
                                }
                                else
                                {
                                    String::new()
                                },
                            ))
                        }
                        _ =>
                        {}
                    }
                }
            }
            _ =>
            {}
        }
        i += 1;
    }
    func
}
pub fn place_funcvar(
    mut funcvar: Vec<(String, Vec<NumStr>)>,
    var: &str,
    num: NumStr,
) -> Vec<(String, Vec<NumStr>)>
{
    if !var.is_empty()
    {
        for i in funcvar.iter_mut()
        {
            if !i.0.contains('(')
            {
                let mut sum = Vec::new();
                let mut bracket = 0;
                let mut j = 0;
                while i.1.len() > j
                {
                    match &i.1[j]
                    {
                        LeftBracket => bracket += 1,
                        RightBracket => bracket -= 1,
                        Comma if sum.contains(&bracket) =>
                        {
                            sum.remove(0);
                        }
                        Func(s) =>
                        {
                            if s == var && sum.is_empty()
                            {
                                i.1[j] = num.clone();
                            }
                            else
                            {
                                match s.as_str()
                                {
                                    "sum" | "summation" | "prod" | "product" | "Σ" | "Π"
                                    | "vec" | "mat" | "D" | "integrate" | "arclength" | "∫"
                                    | "area" | "solve" | "length" | "slope" | "lim" | "limit"
                                    | "set" | "iter" | "extrema" | "surfacearea" | "sarea"
                                        if j + 2 < i.1.len()
                                            && i.1[j + 2] == Func(var.to_string()) =>
                                    {
                                        j += 3;
                                        sum.push(bracket)
                                    }
                                    "surfacearea" | "sarea"
                                        if j + 4 < i.1.len()
                                            && i.1[j + 4] == Func(var.to_string()) =>
                                    {
                                        j += 5;
                                        sum.push(bracket)
                                    }
                                    _ =>
                                    {}
                                }
                            }
                        }
                        _ =>
                        {}
                    }
                    j += 1;
                }
            }
        }
    }
    funcvar
}
pub fn place_var(mut func: Vec<NumStr>, var: &str, num: NumStr) -> Vec<NumStr>
{
    if !var.is_empty()
    {
        let mut sum = Vec::new();
        let mut bracket = 0;
        let mut i = 0;
        while func.len() > i
        {
            match &func[i]
            {
                LeftBracket => bracket += 1,
                RightBracket => bracket -= 1,
                Comma if sum.contains(&bracket) =>
                {
                    sum.remove(0);
                }
                Func(s) =>
                {
                    if s == var && sum.is_empty()
                    {
                        func[i] = num.clone();
                    }
                    else
                    {
                        match s.as_str()
                        {
                            "sum" | "summation" | "prod" | "product" | "Σ" | "Π" | "vec"
                            | "mat" | "D" | "integrate" | "arclength" | "∫" | "area" | "solve"
                            | "length" | "slope" | "lim" | "set" | "limit" | "iter" | "extrema"
                            | "surfacearea" | "sarea"
                                if i + 2 < func.len() && func[i + 2] == Func(var.to_string()) =>
                            {
                                i += 3;
                                sum.push(bracket)
                            }
                            "surfacearea" | "sarea"
                                if i + 4 < func.len() && func[i + 4] == Func(var.to_string()) =>
                            {
                                i += 5;
                                sum.push(bracket)
                            }
                            _ =>
                            {}
                        }
                    }
                }
                _ =>
                {}
            }
            i += 1;
        }
    }
    func
}
pub fn do_math_with_var(
    function: Vec<NumStr>,
    options: Options,
    func_vars: Vec<(String, Vec<NumStr>)>,
    var: &str,
    num: NumStr,
) -> Result<NumStr, &'static str>
{
    do_math(
        place_var(function, var, num.clone()),
        options,
        place_funcvar(func_vars, var, num),
    )
}
pub fn parsed_to_string(
    mut input: Vec<NumStr>,
    vars: &[Variable],
    func_vars: Vec<(String, Vec<NumStr>)>,
    options: &Options,
    colors: &Colors,
) -> String
{
    let mut i = 0;
    'main: while i < input.len()
    {
        if let Func(s) = &input[i]
        {
            for v in &func_vars
            {
                if *s == v.0 && !v.0.ends_with(')')
                {
                    if i != 0
                        && i + 1 < input.len()
                        && input[i - 1] == LeftBracket
                        && input[i + 1] == RightBracket
                    {
                        input.remove(i);
                        input.splice(i..i, v.1.clone());
                    }
                    else
                    {
                        input[i] = LeftBracket;
                        input.splice(i + 1..i + 1, v.1.clone());
                        input.insert(i + v.1.len() + 1, RightBracket);
                    }
                    continue 'main;
                }
            }
        }
        i += 1;
    }
    let mut out = String::new();
    for i in input
    {
        out.push_str(&match i
        {
            Num(n) =>
            {
                let n = custom_units(n, *options, colors);
                let n = get_output(*options, colors, &n);
                format!(
                    "{}{}{}{}",
                    n.0,
                    n.1,
                    n.2.unwrap_or_default(),
                    if options.color == crate::Auto::True
                    {
                        "\x1b[0m"
                    }
                    else
                    {
                        ""
                    }
                )
            }
            Vector(n) =>
            {
                let mut str = String::new();
                let mut num;
                for i in n
                {
                    let i = custom_units(i, *options, colors);
                    num = get_output(*options, colors, &i);
                    str.push_str(&format!(
                        "{}{}{}{},",
                        num.0,
                        num.1,
                        num.2.unwrap_or_default(),
                        if options.color == crate::Auto::True
                        {
                            "\x1b[0m"
                        }
                        else
                        {
                            ""
                        }
                    ));
                }
                str.pop();
                format!("{{{}}}", str)
            }
            Matrix(n) =>
            {
                let mut str = String::new();
                let mut num;
                for i in n
                {
                    str.push('{');
                    for j in i
                    {
                        let j = custom_units(j, *options, colors);
                        num = get_output(*options, colors, &j);
                        str.push_str(&format!(
                            "{}{}{}{},",
                            num.0,
                            num.1,
                            num.2.unwrap_or_default(),
                            if options.color == crate::Auto::True
                            {
                                "\x1b[0m"
                            }
                            else
                            {
                                ""
                            }
                        ));
                    }
                    str.insert(str.len().saturating_sub(1), '}');
                }
                str.pop();
                format!("{{{}}}", str)
            }
            Func(n) if n.starts_with('@') && n.contains('(') =>
            {
                n.split('(').next().unwrap().replace('@', "")
            }
            Func(n) => n.replace('@', ""),
            LeftBracket => "(".to_string(),
            RightBracket => ")".to_string(),
            LeftCurlyBracket => "{".to_string(),
            RightCurlyBracket => "}".to_string(),
            Comma => ",".to_string(),
            Plus => "+".to_string(),
            Minus => "-".to_string(),
            PlusMinus => "±".to_string(),
            Multiplication => "*".to_string(),
            InternalMultiplication => "×".to_string(),
            Division => "/".to_string(),
            Root => "//".to_string(),
            Tetration => "^^".to_string(),
            Exponent => "^".to_string(),
            Equal => "==".to_string(),
            NotEqual => "!=".to_string(),
            Greater => ">".to_string(),
            GreaterEqual => ">=".to_string(),
            Lesser => "<".to_string(),
            LesserEqual => "<".to_string(),
            Modulo => "%".to_string(),
            Range => "..".to_string(),
            And => "&&".to_string(),
            Or => "||".to_string(),
            ShiftLeft => "<<".to_string(),
            ShiftRight => ">>".to_string(),
            Conversion => "->".to_string(),
            NearEqual => "≈".to_string(),
            Xor => " Xor ".to_string(),
            Implies => " Implies ".to_string(),
            Nand => " Nand ".to_string(),
            Not => "¬".to_string(),
            Nor => " Nor ".to_string(),
            Converse => " Converse ".to_string(),
        })
    }
    to_output(
        &out.chars().collect::<Vec<char>>(),
        vars,
        options.color == crate::Auto::True,
        colors,
    )
}
pub fn insert_last(input: &[char], last: &str) -> String
{
    let mut output = String::new();
    let mut word = String::new();
    for c in input
    {
        if c.is_alphanumeric() || matches!(c, '\'' | '`' | '_')
        {
            output.push(*c);
            word.push(*c)
        }
        else
        {
            if word.to_ascii_lowercase() == "ans"
            {
                output.drain(output.len() - 3..);
                output.push('(');
                output.push_str(last);
                output.push(')');
            }
            else if word == "_"
            {
                output.pop();
                output.push('(');
                output.push_str(last);
                output.push(')');
            }
            output.push(*c);
            word.clear()
        }
    }
    if word.to_ascii_lowercase() == "ans"
    {
        output.drain(output.len() - 3..);
        if !last.contains('#')
        {
            output.push('(');
        }
        output.push_str(last);
        if !last.contains('#')
        {
            output.push(')');
        }
    }
    else if word == "_"
    {
        output.pop();
        if !last.contains('#')
        {
            output.push('(');
        }
        output.push_str(last);
        if !last.contains('#')
        {
            output.push(')');
        }
    }
    output
}
