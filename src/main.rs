mod complex;
mod graph;
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
use gnuplot::Figure;
use crate::graph::{get_list_2d, get_list_3d, graph};
use std::f64::consts::PI;
fn main()
{
    let mut range = ([[-10.0, 10.0]; 3], 100000.0, 400.0);
    let mut plot = Figure::new();
    if args().len() > 1
    {
        if args().nth(1).unwrap() == "--help"
        {
            help();
            return;
        }
        let func = match get_func(&args().nth(1).unwrap().replace('z', "(x+y*i)"), true)
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
            if func.contains(&"y".to_string())
            {
                graph(&func, true, true, &mut plot, None, range);
                return;
            }
            graph(&func, false, true, &mut plot, None, range);
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
    let mut older:Vec<[Vec<[f64; 2]>; 2]> = Vec::new();
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
                    loop
                    {
                        i -= 1;
                        if i == -1
                        {
                            input.clear();
                            i = 0;
                            continue 'outer;
                        }
                        if input == lines[i as usize]
                        {
                            continue;
                        }
                        input = lines[i as usize].clone();
                        cursor = input.len();
                        frac = print_concurrent(&input, var.clone(), false, &mut last, frac);
                        break;
                    }
                    print!("\x1B[2K\x1B[1G{fg}{}\x1b[0m", input);
                }
                '\x1E' =>
                {
                    loop
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
                        if input == lines[i as usize]
                        {
                            continue;
                        }
                        input = lines[i as usize].clone();
                        cursor = input.len();
                        frac = print_concurrent(&input, var.clone(), false, &mut last, frac);
                        break;
                    }
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
            plot.clear_axes();
            older.clear();
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
            file.write_all(input.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
            let l = input.split('=').next().unwrap();
            let r = input.split('=').last().unwrap();
            if l == "zrange"
            {
                range.0[2][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                range.0[2][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                continue;
            }
            if l == "yrange"
            {
                range.0[1][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                range.0[1][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                continue;
            }
            if l == "xrange"
            {
                range.0[0][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                range.0[0][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                continue;
            }
            if l == "3d"
            {
                range.2 = r.parse::<f64>().unwrap();
                continue;
            }
            if l == "2d"
            {
                range.1 = r.parse::<f64>().unwrap();
                continue;
            }
            if input.contains('x') && var.iter().all(|i| i[0] != 'x') && l.contains('x') && r.contains('x')
            {
                let lf = match get_func(l, true)
                {
                    Ok(f) => f,
                    _ => continue,
                };
                let rf = match get_func(r, true)
                {
                    Ok(f) => f,
                    _ => continue,
                };
                if input.contains('y') && var.iter().all(|i| i[0] != 'y') && l.contains('y') && r.contains('y')
                {
                    if get_list_3d(&lf, range) == get_list_3d(&rf, range)
                    {
                        println!("true");
                    }
                    else
                    {
                        println!("false");
                    }
                    continue;
                }
                if get_list_2d(&lf, range) == get_list_2d(&rf, range)
                {
                    println!("true");
                }
                else
                {
                    println!("false");
                }
                continue;
            }
            if l.len() > 1
            {
                let (re, im) = parse(&r.to_string());
                let mut list:Vec<f64> = Vec::new();
                let l = get_list_2d(&match get_func(l, true)
                                    {
                                        Ok(f) => f,
                                        _ => continue,
                                    },
                                    range);
                l.0.iter().for_each(|i| {
                              if i[1] == re
                              {
                                  list.push(i[0]);
                              }
                          });
                l.1.iter().for_each(|i| {
                              if i[1] == im && list.contains(&i[0])
                              {
                                  println!("{}", i[0])
                              }
                          });
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
            input = input.replace('z', "(x+y*i)");
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            file.write_all(input.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
            let func = match get_func(&input, true)
            {
                Ok(f) => f,
                _ => continue,
            };
            if input.contains('y')
            {
                graph(&func, true, false, &mut plot, None, range);
                continue;
            }
            let data = graph(&func, false, false, &mut plot, Some(older.clone()), range);
            if let Some(..) = data
            {
                older.push(data.unwrap());
            }
            continue;
        }
        file.write_all(input.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
        println!();
    }
}
fn help()
{
    println!("Type in a function to evaluate it. Type \"exit\" to exit. Type \"clear\" to clear the screen. Type \"help\" to show this message.");
    println!("functions: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, sqrt, cbrt, ln, log(base,num),root(base,exp), abs, sgn, arg, dg(to_degrees),rd(to_radians),re(real part),im(imaginary part)");
}
fn print_answer(func:Vec<String>)
{
    let num = match do_math(func)
    {
        Ok(num) => num,
        Err(_) =>
        {
            println!("\x1b[91m0\x1b[0m");
            return;
        }
    };
    let (a, b) = parse(&num);
    let a = (a * 1e12).round() / 1e12;
    let b = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &((b * 1e12).round() / 1e12).to_string() + "\x1b[93mi";
    println!("{}{}\x1b[0m",
             if a == 0.0 && !(b.ends_with("0\x1b[93mi")) { "".to_string() } else { a.to_string() },
             if b.ends_with("0\x1b[93mi") { "".to_string() } else { b });
}
fn fraction(num:f64) -> String
{
    if (num * 1e12).round() / 1e12 == 0.0 || ((num * 1e12).round() / 1e12).fract() == 0.0
    {
        return "0".to_string();
    }
    let mut p;
    let sqrt2 = 2f64.sqrt();
    for i in 1..=10
    {
        p = (1e12 * num / sqrt2 * i as f64).round() / 1e12;
        if p.fract() == 0.0
        {
            return format!("{}sqrt(2){}",
                           if p == 1.0
                           {
                               "".to_string()
                           }
                           else if p == -1.0
                           {
                               "-".to_string()
                           }
                           else
                           {
                               p.to_string()
                           },
                           if i == 1 { "".to_string() } else { format!("/{}", i) });
        }
    }
    let sqrt3 = 3f64.sqrt();
    for i in 1..=10
    {
        p = (1e12 * num / sqrt3 * i as f64).round() / 1e12;
        if p.fract() == 0.0
        {
            return format!("{}sqrt(3){}",
                           if p == 1.0
                           {
                               "".to_string()
                           }
                           else if p == -1.0
                           {
                               "-".to_string()
                           }
                           else
                           {
                               p.to_string()
                           },
                           if i == 1 { "".to_string() } else { format!("/{}", i) });
        }
    }
    for i in 1..=10
    {
        p = num / PI * i as f64;
        if p.fract() == 0.0
        {
            return format!("{}π{}",
                           if p == 1.0
                           {
                               "".to_string()
                           }
                           else if p == -1.0
                           {
                               "-".to_string()
                           }
                           else
                           {
                               p.to_string()
                           },
                           if i == 1 { "".to_string() } else { format!("/{}", i) });
        }
    }
    let mut i = 1;
    while i <= 10000
    {
        let product = num * i as f64;
        if product.fract() == 0.0
        {
            let denominator = if i == 1 { "".to_string() } else { format!("/{}", i) };
            return format!("{}{}", product, denominator);
        }
        i += 1;
        if i <= 10000 && product > i as f64 * num && product.is_sign_positive()
        {
            i = (product / num) as usize;
        }
    }
    num.to_string()
}
fn print_concurrent(input:&String, var:Vec<Vec<char>>, del:bool, last:&mut Vec<String>, frac:bool) -> bool
{
    let mut modified = input.to_string();
    for i in &var
    {
        let var_name = i[0..i.iter().position(|&x| x == '=').unwrap()].iter().collect::<String>();
        let var_value = i[i.iter().position(|&x| x == '=').unwrap() + 1..].iter().collect::<String>();
        let var_name_len = var_name.len();
        let mut start_idx = 0;
        while start_idx < modified.len() + 1 - var_name_len
        {
            let end_idx = start_idx + var_name_len;
            if (start_idx == 0 || !modified.chars().nth(start_idx - 1).unwrap().is_ascii_alphabetic())
               && (end_idx == modified.len() || !modified.chars().nth(end_idx).unwrap().is_ascii_alphabetic())
               && modified[start_idx..end_idx] == var_name
            {
                modified.replace_range(start_idx..end_idx, &var_value);
            }
            start_idx += 1;
        }
    }
    if modified.contains('x') || modified.contains('y') || modified.contains('z')
    {
        print!("\x1b[96m\x1b[B\x1B[2K\x1B[1G\x1b[A\x1B[2K\x1B[1G{}\x1b[0m", input);
        return false;
    }
    let func = match get_func(&modified, false)
    {
        Ok(f) => f,
        Err(_) =>
        {
            print!("\x1b[96m\x1B[2K\x1B[1G{}\x1b[0m", input);
            return false;
        }
    };
    if func == *last
    {
        print!("\x1b[96m\x1B[2K\x1B[1G{}\x1b[0m", input);
        return frac;
    }
    let mut frac = false;
    *last = func.clone();
    if let Ok(num) = do_math(func.clone())
    {
        if num == "inf" || num == "-inf" || num == "nan"
        {
            print!("\x1b[0m\x1b[B\x1B[2K\x1B[1G{}\x1b[0m\x1b[A", num);
            print!("\x1b[96m\x1B[2K\x1B[1G{}\x1b[0m", input);
            return false;
        }
        let (a, b) = parse(&num);
        let c = (a * 1e12).round() / 1e12;
        let sign = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned();
        let d = (b * 1e12).round() / 1e12;
        let fa = fraction(a);
        let fb = fraction(b);
        if (fa.contains('/') && fb.contains('/')) || (fa.contains('π') && fb.contains('π') || (fa.contains('s') && fb.contains('s')))
        {
            frac = true;
            print!("\x1b[0m\x1b[B\x1B[2K\x1B[1G{}{}",
                   if c == 0.0 && d != 0.0 { "".to_string() } else { fa },
                   if d == 0.0 { "".to_string() } else { sign.clone() + fb.as_str() + "\x1b[93mi" });
        }
        else if (fa.contains('/') || fa.contains('π') || fa.contains('s')) && fa != func.join("")
        {
            frac = true;
            print!("\x1b[0m\x1b[B\x1B[2K\x1B[1G{}{}",
                   if c == 0.0 && d != 0.0 { "".to_string() } else { fa },
                   if d == 0.0 { "".to_string() } else { sign.clone() + d.to_string().as_str() + "\x1b[93mi" });
        }
        else if (fb.contains('/') || fb.contains('π') || fa.contains('s')) && fb != func.join("")
        {
            frac = true;
            print!("\x1b[0m\x1b[B\x1B[2K\x1B[1G{}{}",
                   if c == 0.0 && d != 0.0 { "".to_string() } else { c.to_string() },
                   if d == 0.0 { "".to_string() } else { sign.clone() + fb.as_str() + "\x1b[93mi" });
        }
        if !frac
        {
            print!("\x1b[B\x1b[B\x1B[2K\x1B[1G\x1b[A\x1b[A");
        }
        print!("\x1b[0m\x1b[B\x1B[2K\x1B[1G{}{}\x1b[A",
               if c == 0.0 && d != 0.0 { "".to_string() } else { c.to_string() },
               if d == 0.0 { "".to_string() } else { sign + d.to_string().as_str() + "\x1b[93mi" });
        if frac
        {
            print!("\x1b[A");
        }
    }
    if !del
    {
        print!("\x1b[96m\x1B[2K\x1B[1G{}\x1b[0m", input);
    }
    else
    {
        print!("\x1B[2K\x1B[1G");
    }
    frac
}
fn read_single_char() -> char
{
    let term = Term::stdout();
    let key = term.read_key().unwrap();
    match key
    {
        Key::Char(c) =>
        {
            if c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '*' || c == '/' || c == '^' || c == '(' || c == ')' || c == '.' || c == '=' || c == ',' || c == 'π'
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