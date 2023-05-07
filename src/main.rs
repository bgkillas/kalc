mod complex;
mod math;
mod parse;
use parse::get_func;
use complex::parse;
use math::do_math;
use std::env::{args, var};
use std::io::{BufRead, BufReader, stdout, Write};
use console::{Key, Term};
#[cfg(target_os = "linux")]
use std::io::stdin;
#[cfg(target_os = "linux")]
use libc::{isatty, STDIN_FILENO};
use std::fs::{File, OpenOptions};
fn print_answer(func:Vec<String>)
{
    let num = do_math(func);
    let (a, b) = parse(&num);
    let a = (a * 1e9).round() / 1e9;
    let b = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &((b * 1e9).round() / 1e9).to_string() + "i";
    println!("{}{}",
             if a == 0.0 && !(b == "-0i" || b == "+0i" || b == "0i") { "".to_string() } else { a.to_string() },
             if b == "-0i" || b == "+0i" || b == "0i" { "".to_string() } else { b });
}
fn write_history(input:&str, file_path:&str)
{
    let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
    file.write_all(input.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
}
fn read_single_char() -> char
{
    let term = Term::stdout();
    let key = term.read_key().unwrap();
    match key
    {
        Key::Char(c) =>
        {
            if c == ' '
            {
                read_single_char()
            }
            else
            {
                c
            }
        }
        Key::Enter => '\n',
        Key::Backspace => '\x08',
        Key::ArrowLeft => '\x1B',
        Key::ArrowRight => '\x1C',
        Key::ArrowUp => '\x1D',
        Key::ArrowDown => '\x1E',
        _ => read_single_char(),
    }
}
fn main()
{
    if args().len() > 1
    {
        if args().nth(1).unwrap() == "--help"
        {
            println!("Type in a function to evaluate it. Type \"exit\" to exit. Type \"clear\" to clear the screen. Type \"help\" to show this message.");
            println!("functions: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, sqrt, cbrt, ln, log(base,num), abs, dg(to_degrees),rd(to_radians)");
            return;
        }
        let func = get_func(args().nth(1).unwrap());
        if func.contains(&"x".to_string())
        {
            if func.contains(&"y".to_string())
            {
                let mut modified;
                for n in -100..=100
                {
                    modified = func.clone();
                    modified = modified.iter().map(|i| i.replace('x', &(n as f64 / 10.0).to_string())).collect();
                    for g in -100..=100
                    {
                        let num = do_math(modified.iter().map(|j| j.replace('y', &(g as f64 / 10.0).to_string())).collect());
                        let (a, b) = parse(&num);
                        let a = (a * 1e9).round() / 1e9;
                        let b = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &((b * 1e9).round() / 1e9).to_string() + "i";
                        println!("{} {} {} {}",
                                 n as f64 / 10.0,
                                 g as f64 / 10.0,
                                 if a == 0.0 && !(b == "-0i" || b == "+0i" || b == "0i") { "".to_string() } else { a.to_string() },
                                 if b == "-0i" || b == "+0i" || b == "0i" { "".to_string() } else { b });
                    }
                }
                return;
            }
            let mut modified;
            for n in -100000..=100000
            {
                modified = func.clone();
                for i in &mut modified
                {
                    if i == "x"
                    {
                        *i = (n as f64 / 10000.0).to_string();
                    }
                }
                let num = do_math(modified);
                let (a, b) = parse(&num);
                let a = (a * 1e9).round() / 1e9;
                let b = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &((b * 1e9).round() / 1e9).to_string() + "i";
                println!("{} {} {}",
                         n as f64 / 10000.0,
                         if a == 0.0 && !(b == "-0i" || b == "+0i" || b == "0i") { "".to_string() } else { a.to_string() },
                         if b == "-0i" || b == "+0i" || b == "0i" { "".to_string() } else { b });
            }
            return;
        }
        print_answer(func);
        return;
    }
    let mut input = String::new();
    #[cfg(target_os = "linux")]
    if !unsafe { isatty(STDIN_FILENO) != 0 }
    {
        let line = stdin().lock().lines().next();
        if line.as_ref().is_none()
        {
            return;
        }
        input = line.unwrap().unwrap();
        if input.is_empty()
        {
            return;
        }
        print_answer(get_func(input));
        return;
    }
    #[cfg(target_os = "linux")]
    let file_path = &(var("HOME").unwrap() + "/.config/calc.history");
    #[cfg(target_os = "windows")]
    let file_path = &format!("C:\\Users\\{}\\AppData\\Roaming\\calc.history", var("USERNAME").unwrap());
    if File::open(file_path).is_err()
    {
        File::create(file_path).unwrap();
    }
    let mut var:Vec<Vec<char>> = Vec::new();
    loop
    {
        input.clear();
        let fg = "\x1b[38;2;108;149;235m";
        print!("{fg}");
        stdout().flush().unwrap();
        let mut i = BufReader::new(File::open(file_path).unwrap()).lines().count() as i32;
        let max = i;
        let mut cursor = 0;
        loop
        {
            let c = read_single_char();
            match c
            {
                '\n' =>
                {
                    println!("\x1b[0m");
                    break;
                }
                '\x08' =>
                {
                    if input.is_empty()
                    {
                        continue;
                    }
                    cursor -= 1;
                    input.remove(cursor);
                    print!("\x1B[2K\x1B[1G{}", input);
                    for _ in 0..(input.len() - cursor)
                    {
                        print!("\x08");
                    }
                }
                '\x1D' =>
                {
                    i -= 1;
                    input.clear();
                    if i == -1
                    {
                        i = 0;
                        continue;
                    }
                    input = BufReader::new(File::open(file_path).unwrap()).lines().nth(i as usize).unwrap().unwrap();
                    cursor = input.len();
                    print!("\x1B[2K\x1B[1G{fg}{}", input);
                }
                '\x1E' =>
                {
                    i += 1;
                    input.clear();
                    if i >= max
                    {
                        print!("\x1B[2K\x1B[1G{fg}");
                        stdout().flush().unwrap();
                        i = max;
                        cursor = 0;
                        continue;
                    }
                    input = BufReader::new(File::open(file_path).unwrap()).lines().nth(i as usize).unwrap().unwrap();
                    cursor = input.len();
                    print!("\x1B[2K\x1B[1G{fg}{}", input);
                }
                '\x1B' =>
                {
                    if cursor == 0
                    {
                        continue;
                    }
                    cursor -= 1;
                    print!("\x08");
                }
                '\x1C' =>
                {
                    if cursor == input.len()
                    {
                        continue;
                    }
                    cursor += 1;
                    print!("\x1b[1C")
                }
                _ =>
                {
                    input.insert(cursor, c);
                    cursor += 1;
                    print!("\x1B[2K\x1B[1G{}", input);
                    for _ in 0..(input.len() - cursor)
                    {
                        print!("\x08");
                    }
                }
            }
            stdout().flush().unwrap();
        }
        if input.contains('=')
        {
            for i in 0..var.len()
            {
                if var[i][0] == input.chars().next().unwrap()
                {
                    var.remove(i);
                    break;
                }
            }
            var.push(input.chars().collect());
            write_history(&input, file_path);
            continue;
        }
        if input == "exit"
        {
            break;
        }
        if input == "clear"
        {
            print!("\x1B[2J\x1B[1;1H");
            stdout().flush().unwrap();
            continue;
        }
        if input == "help"
        {
            println!("Type in a function to evaluate it. Type \"exit\" to exit. Type \"clear\" to clear the screen. Type \"help\" to show this message.");
            println!("functions: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, sqrt, cbrt, ln, log(base,num), abs, dg(to_degrees),rd(to_radians)");
            continue;
        }
        if input.is_empty()
        {
            continue;
        }
        let unmodified = input.clone();
        for i in &var
        {
            input = input.replace(&i[0..i.iter().position(|&x| x == '=').unwrap()].iter().collect::<String>(),
                                  &i[i.iter().position(|&x| x == '=').unwrap() + 1..].iter().collect::<String>());
        }
        if input.contains('x') || input.contains('y')
        {
            println!("{}", input);
            write_history(&input, file_path);
            continue;
        }
        write_history(&unmodified, file_path);
        print_answer(get_func(input.clone()));
    }
}