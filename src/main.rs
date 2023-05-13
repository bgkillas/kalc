mod complex;
mod fraction;
mod graph;
mod math;
mod parse;
mod print;
use parse::get_func;
use std::env::{args, var};
use std::io::{BufRead, BufReader, stdout, Write};
use console::{Key, Term};
use std::fs::{File, OpenOptions};
use gnuplot::Figure;
use crate::graph::{get_list_2d, graph};
use crate::print::{print_answer, print_concurrent};
#[cfg(target_os = "linux")]
use {
    std::io::stdin, libc::{isatty, STDIN_FILENO}
};
use crate::math::do_math;
fn help()
{
    println!("Type in a function to evaluate it. Type \"exit\" to exit. Type \"clear\" to clear the screen. Type \"help\" to show this message.");
    println!(
             "functions: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh,\n\
    csc, sec, cot, acsc, asec, acot, csch, sech, coth, acsch, asech, acoth,\n\
    sqrt, cbrt, ln, log(base,num), root(base,exp), exp, abs, sgn, arg, ceil, floor, round,\n\
    deg(to_degrees),rad(to_radians),re(real part),im(imaginary part)"
    );
}
fn write(input:&String, file:&mut File, lines:&Vec<String>)
{
    if lines.is_empty() || lines[lines.len() - 1] != *input
    {
        file.write_all(input.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}
fn main()
{
    let mut range = ([[-10.0, 10.0]; 3], 20000.0, 400.0);
    let mut plot = Figure::new();
    plot.set_enhanced_text(false);
    if args().len() > 1
    {
        if args().nth(1).unwrap() == "--help"
        {
            help();
            return;
        }
        let input = &args().nth(1).unwrap().replace('z', "(x+y*i)");
        let func = match get_func(input, true)
        {
            Ok(f) => f,
            Err(()) =>
            {
                println!("Invalid function.");
                return;
            }
        };
        if func.contains(&"x".to_string())
        {
            let mut split = input.split('#');
            let l = split.next().unwrap();
            let m = split.next().unwrap_or("0");
            let r = split.next().unwrap_or("0");
            let funcl = match get_func(l, true)
            {
                Ok(f) => f,
                _ =>
                {
                    println!("Invalid function.");
                    return;
                }
            };
            let funcm = match get_func(m, true)
            {
                Ok(f) => f,
                _ =>
                {
                    println!("Invalid function.");
                    return;
                }
            };
            let funcr = match get_func(r, true)
            {
                Ok(f) => f,
                _ =>
                {
                    println!("Invalid function.");
                    return;
                }
            };
            if func.contains(&"y".to_string())
            {
                graph([&l.to_string(), &m.to_string(), &r.to_string()], [&funcl, &funcm, &funcr], true, true, &mut plot, range);
                return;
            }
            graph([&l.to_string(), &m.to_string(), &r.to_string()], [&funcl, &funcm, &funcr], false, true, &mut plot, range);
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
        print_answer(match get_func(&input, true)
        {
            Ok(f) => f,
            Err(()) =>
            {
                println!("Invalid function.");
                return;
            }
        });
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
    let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
    loop
    {
        input.clear();
        let fg = "\x1b[96m";
        stdout().flush().unwrap();
        let lines:Vec<String> = BufReader::new(File::open(file_path).unwrap()).lines().map(|l| l.unwrap()).collect();
        let mut i = lines.len() as i32;
        let max = i;
        let mut cursor = 0;
        let mut frac = false;
        let lines = lines;
        let mut last:Vec<String> = Vec::new();
        'outer: loop
        {
            let c = read_single_char();
            match c
            {
                '\n' =>
                {
                    if frac
                    {
                        println!();
                    }
                    println!();
                    break;
                }
                '\x08' =>
                {
                    if cursor == 0
                    {
                        if input.is_empty()
                        {
                            print!("\x1b[B\x1B[2K\x1B[1G\x1b[B\x1B[2K\x1B[1G\x1b[A\x1b[A");
                            stdout().flush().unwrap();
                        }
                        continue;
                    }
                    cursor -= 1;
                    input.remove(cursor);
                    print!("\x1B[2K\x1B[1G{}", input);
                    if input.is_empty()
                    {
                        frac = print_concurrent(&"0".to_string(), var.clone(), true, &mut last, frac);
                    }
                    else
                    {
                        frac = print_concurrent(&input, var.clone(), false, &mut last, frac);
                    }
                    for _ in 0..(input.len() - cursor)
                    {
                        print!("\x08");
                    }
                    if cursor == 0 && input.is_empty()
                    {
                        print!("\x1B[2K\x1B[1G\x1b[B\x1B[2K\x1B[1G\x1b[B\x1B[2K\x1B[1G\x1b[A\x1b[A");
                    }
                }
                '\x1D' =>
                {
                    i -= 1;
                    if i == -1
                    {
                        input.clear();
                        i = 0;
                        continue 'outer;
                    }
                    input = lines[i as usize].clone();
                    cursor = input.len();
                    frac = print_concurrent(&input, var.clone(), false, &mut last, frac);
                    print!("\x1B[2K\x1B[1G{fg}{}\x1b[0m", input);
                }
                '\x1E' =>
                {
                    i += 1;
                    if i >= max
                    {
                        input.clear();
                        print!("\x1b[B\x1B[2K\x1B[1G\x1b[B\x1B[2K\x1B[1G\x1b[A\x1b[A");
                        print!("\x1B[2K\x1B[1G{fg}\x1b[0m");
                        stdout().flush().unwrap();
                        i = max;
                        cursor = 0;
                        continue 'outer;
                    }
                    input = lines[i as usize].clone();
                    cursor = input.len();
                    frac = print_concurrent(&input, var.clone(), false, &mut last, frac);
                    print!("\x1B[2K\x1B[1G{fg}{}\x1b[0m", input);
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
                    if c == 'π'
                    {
                        input.insert(cursor, 'p');
                        cursor += 1;
                        input.insert(cursor, 'i');
                        cursor += 1;
                    }
                    else
                    {
                        input.insert(cursor, c);
                        cursor += 1;
                    }
                    frac = print_concurrent(&input, var.clone(), false, &mut last, frac);
                    for _ in 0..(input.len() - cursor)
                    {
                        print!("\x08");
                    }
                }
            }
            stdout().flush().unwrap();
        }
        if input == "exit"
        {
            break;
        }
        if input == "clear"
        {
            plot.close();
            plot.clear_axes();
            print!("\x1B[2J\x1B[1;1H");
            stdout().flush().unwrap();
            continue;
        }
        if input == "help"
        {
            help();
            continue;
        }
        if input.is_empty()
        {
            continue;
        }
        if input.contains('=')
        {
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            write(&input, &mut file, &lines);
            let l = input.split('=').next().unwrap();
            let r = input.split('=').last().unwrap();
            match l
            {
                "xrange" =>
                {
                    range.0[0][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                    range.0[0][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                    continue;
                }
                "yrange" =>
                {
                    range.0[1][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                    range.0[1][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                    continue;
                }
                "zrange" =>
                {
                    range.0[2][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                    range.0[2][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                    continue;
                }
                "2d" =>
                {
                    range.1 = r.parse::<f64>().unwrap();
                    continue;
                }
                "3d" =>
                {
                    range.2 = r.parse::<f64>().unwrap();
                    continue;
                }
                _ => (),
            }
            if input.contains('x')
            {
                let l = match get_func(l, true)
                {
                    Ok(i) => i,
                    Err(()) =>
                    {
                        continue;
                    }
                };
                let r = match get_func(r, true)
                {
                    Ok(i) => i,
                    Err(()) =>
                    {
                        continue;
                    }
                };
                let (lre, lim) = get_list_2d(&l, range);
                let (rre, rim) = get_list_2d(&r, range);
                let mut success = true;
                for i in 0..lre.len()
                {
                    if (lre[i][1] * 1e9).round() / 1e9 != (rre[i][1] * 1e9).round() / 1e9 || (lim[i][1] * 1e9).round() / 1e9 != (rim[i][1] * 1e9).round() / 1e9
                    {
                        success = false;
                    }
                }
                if success
                {
                    println!("true");
                    continue;
                }
                println!("false");
                continue;
            }
            if l.len() > 1
            {
                let l = match do_math(match get_func(l, false)
                      {
                          Ok(i) => i,
                          Err(()) =>
                          {
                              continue;
                          }
                      })
                {
                    Ok(i) => i,
                    Err(()) =>
                    {
                        continue;
                    }
                };
                let r = match do_math(match get_func(r, false)
                      {
                          Ok(i) => i,
                          Err(()) =>
                          {
                              continue;
                          }
                      })
                {
                    Ok(i) => i,
                    Err(()) =>
                    {
                        continue;
                    }
                };
                if (l.parse::<f64>().unwrap() * 1e12).round() / 1e12 == (r.parse::<f64>().unwrap() * 1e12).round() / 1e12
                {
                    println!("true");
                    continue;
                }
                println!("false");
                continue;
            }
            for i in 0..var.len()
            {
                if var[i][0] == input.chars().next().unwrap()
                {
                    var.remove(i);
                    break;
                }
            }
            var.push(input.chars().collect());
            continue;
        }
        else if (input.contains('x') && var.iter().all(|i| i[0] != 'x')) || (input.contains('z') && var.iter().all(|i| i[0] != 'z'))
        {
            write(&input, &mut file, &lines);
            input = input.replace('z', "(x+y*i)");
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            let mut split = input.split('#');
            let l = split.next().unwrap();
            let m = split.next().unwrap_or("0");
            let r = split.next().unwrap_or("0");
            let funcl = match get_func(l, true)
            {
                Ok(f) => f,
                _ => continue,
            };
            let funcm = match get_func(m, true)
            {
                Ok(f) => f,
                _ => continue,
            };
            let funcr = match get_func(r, true)
            {
                Ok(f) => f,
                _ => continue,
            };
            if input.contains('y')
            {
                graph([&l.to_string(), &m.to_string(), &r.to_string()], [&funcl, &funcm, &funcr], true, false, &mut plot, range);
                continue;
            }
            graph([&l.to_string(), &m.to_string(), &r.to_string()], [&funcl, &funcm, &funcr], false, false, &mut plot, range);
            continue;
        }
        write(&input, &mut file, &lines);
        println!();
    }
}
fn read_single_char() -> char
{
    let term = Term::stdout();
    let key = term.read_key().unwrap();
    match key
    {
        Key::Char(c) =>
        {
            if c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '*' || c == '/' || c == '^' || c == '(' || c == ')' || c == '.' || c == '=' || c == ',' || c == 'π' || c == '#' || c == '|'
            {
                c
            }
            else
            {
                read_single_char()
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