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
use gnuplot::{AxesCommon, Figure, Fix, PointSymbol};
fn main()
{
    let mut range = ([[-10.0, 10.0]; 3], 100000.0, 400.0);
    let mut plot = Figure::new();
    if args().len() > 1
    {
        if args().nth(1).unwrap() == "--help"
        {
            println!("Type in a function to evaluate it. Type \"exit\" to exit. Type \"clear\" to clear the screen. Type \"help\" to show this message.");
            println!("functions: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, sqrt, cbrt, ln, log(base,num), abs, dg(to_degrees),rd(to_radians)");
            return;
        }
        let func = get_func(&args().nth(1).unwrap(), true);
        if func.contains(&"x".to_string())
        {
            if func.contains(&"y".to_string())
            {
                let mut re3d = true;
                let mut im3d = true;
                for i in 2..args().len()
                {
                    if args().nth(i).unwrap() == "--nore"
                    {
                        re3d = false;
                    }
                    if args().nth(i).unwrap() == "--noim"
                    {
                        im3d = false;
                    }
                }
                graph(&func, true, im3d, re3d, &mut plot, None, range);
                return;
            }
            graph(&func, false, true, false, &mut plot, None, range);
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
        print_answer(get_func(&input, true));
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
    loop
    {
        input.clear();
        let fg = "\x1b[96m";
        stdout().flush().unwrap();
        let mut i = BufReader::new(File::open(file_path).unwrap()).lines().count() as i32;
        let max = i;
        let mut cursor = 0;
        let mut frac = false;
        loop
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
                    if input.is_empty() || cursor == 0
                    {
                        print!("\x1b[B\x1B[2K\x1B[1G\x1b[B\x1B[2K\x1B[1G\x1b[A\x1b[A");
                        stdout().flush().unwrap();
                        continue;
                    }
                    cursor -= 1;
                    input.remove(cursor);
                    print!("\x1B[2K\x1B[1G{}", input);
                    if input.is_empty()
                    {
                        frac = print_concurrent(&"0".to_string(), var.clone(), true);
                    }
                    else
                    {
                        frac = print_concurrent(&input, var.clone(), false);
                    }
                    for _ in 0..(input.len() - cursor)
                    {
                        print!("\x08");
                    }
                    if cursor == 0
                    {
                        print!("\x1B[2K\x1B[1G\x1b[B\x1B[2K\x1B[1G\x1b[B\x1B[2K\x1B[1G\x1b[A\x1b[A");
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
                    frac = print_concurrent(&input, var.clone(), false);
                    print!("\x1B[2K\x1B[1G{fg}{}\x1b[0m", input);
                }
                '\x1E' =>
                {
                    i += 1;
                    input.clear();
                    if i >= max
                    {
                        print!("\x1b[B\x1B[2K\x1B[1G\x1b[B\x1B[2K\x1B[1G\x1b[A\x1b[A");
                        print!("\x1B[2K\x1B[1G{fg}\x1b[0m");
                        stdout().flush().unwrap();
                        i = max;
                        cursor = 0;
                        continue;
                    }
                    input = BufReader::new(File::open(file_path).unwrap()).lines().nth(i as usize).unwrap().unwrap();
                    cursor = input.len();
                    frac = print_concurrent(&input, var.clone(), false);
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
                    frac = print_concurrent(&input, var.clone(), false);
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
            println!("Type in a function to evaluate it. Type \"exit\" to exit. Type \"clear\" to clear the screen. Type \"help\" to show this message.");
            println!("functions: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, sqrt, cbrt, ln, log(base,num), abs, dg(to_degrees),rd(to_radians)");
            continue;
        }
        if input.is_empty()
        {
            continue;
        }
        if !input.contains('=') && (input.contains('x') && var.iter().all(|i| i[0] != 'x')) || (input.contains('z') && var.iter().all(|i| i[0] != 'z'))
        {
            input = input.replace('z', "x+y*i");
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            write_history(&input, file_path);
            if input.contains('y')
            {
                graph(&get_func(&input, true), true, true, true, &mut plot, None, range);
                continue;
            }
            let data = graph(&get_func(&input, true), false, false, false, &mut plot, Some(older.clone()), range);
            if let Some(..) = data
            {
                older.push(data.unwrap());
            }
            continue;
        }
        if input.contains('=')
        {
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            write_history(&input, file_path);
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
                if input.contains('y') && var.iter().all(|i| i[0] != 'y') && l.contains('y') && r.contains('y')
                {
                    if get_list_3d(&get_func(l, true), range) == get_list_3d(&get_func(r, true), range)
                    {
                        println!("true");
                    }
                    else
                    {
                        println!("false");
                    }
                    continue;
                }
                if get_list_2d(&get_func(l, true), range) == get_list_2d(&get_func(r, true), range)
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
                let l = get_list_2d(&get_func(l, true), range);
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
        write_history(&input, file_path);
        println!();
    }
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
use std::f64::consts::PI;
use std::f64::consts::FRAC_1_SQRT_2;
fn fraction(num:f64) -> String
{
    if (num * 1e12).round() / 1e12 == 0.0 || ((num * 1e12).round() / 1e12).fract() == 0.0
    {
        return "0".to_string();
    }
    if (num * 1e12).round() / 1e12 == (1e12 * FRAC_1_SQRT_2).round() / 1e12
    {
        return "1/2^0.5".to_string();
    }
    if (num * 1e12).round() / 1e12 == (1e12 * 3f64.sqrt() / 2.0).round() / 1e12
    {
        return "3^0.5/2".to_string();
    }
    let mut p;
    for i in 1..=100
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
// fn simplify(func:Vec<String>) -> String
// {
//     let mut func = func;
//     let mut i = 0;
//     while i < func.len()
//     {
//         if let Ok(num) = func[i].parse::<f64>()
//         {
//             if num.fract() != 0.0
//             {
//                 let frac = fraction(num);
//                 let frac = frac.split('/').collect::<Vec<&str>>();
//                 if frac.contains(&'/'.to_string().as_str())
//                 {
//                     func[i] = "(".to_string();
//                     func.insert(i + 1, frac[0].to_string());
//                     func.insert(i + 2, "/".to_string());
//                     func.insert(i + 3, frac[1].to_string());
//                     func.insert(i + 4, ")".to_string());
//                 }
//             }
//         }
//         if i != 0 && i + 2 < func.len() && func[i + 1] == "*" && func[i - 1] == "/" && func[i + 2] == func[i]
//         {
//             func.remove(i + 1);
//             func.remove(i + 1);
//             func.remove(i - 1);
//             func.remove(i - 1);
//             i -= 1;
//         }
//         i += 1;
//     }
//     func.join("")
// }
fn print_concurrent(input:&String, var:Vec<Vec<char>>, del:bool) -> bool
{
    let mut frac = false;
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
    let func = get_func(&modified, false);
    if let Ok(num) = do_math(func.clone())
    {
        let (a, b) = parse(&num);
        let c = (a * 1e12).round() / 1e12;
        let sign = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned();
        let d = (b * 1e12).round() / 1e12;
        let fa = fraction(a);
        let fb = fraction(b);
        // let simplified = simplify(func.clone());
        // let mut is_simple = false;
        // if simplified != func.join("")
        // {
        //     is_simple = true;
        //     print!("\x1b[B\x1B[2K\x1B[1G{}", simplified);
        //     output += 1;
        // }
        if (fa.contains('/') && fb.contains('/')) || (fa.contains('π') && fb.contains('π'))
        {
            frac = true;
            print!("\x1b[0m\x1b[B\x1B[2K\x1B[1G{}{}",
                   if c == 0.0 && d != 0.0 { "".to_string() } else { fa },
                   if d == 0.0 { "".to_string() } else { sign.clone() + fb.as_str() + "\x1b[93mi" });
        }
        else if (fa.contains('/') || fa.contains('π')) && fa != func.join("")
        {
            frac = true;
            print!("\x1b[0m\x1b[B\x1B[2K\x1B[1G{}{}",
                   if c == 0.0 && d != 0.0 { "".to_string() } else { fa },
                   if d == 0.0 { "".to_string() } else { sign.clone() + d.to_string().as_str() + "\x1b[93mi" });
        }
        else if (fb.contains('/') || fb.contains('π')) && fb != func.join("")
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
        // if is_simple
        // {
        //     print!("\x1b[A");
        // }
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
fn get_list_3d(func:&[String], range:([[f64; 2]; 3], f64, f64)) -> (Vec<[f64; 3]>, Vec<[f64; 3]>)
{
    let mut re = Vec::new();
    let mut im = Vec::new();
    let den = range.2;
    let min_x = range.0[0][0];
    let max_x = range.0[0][1];
    let den_x_range = (max_x - min_x) / den;
    let min_y = range.0[1][0];
    let max_y = range.0[1][1];
    let den_y_range = (max_y - min_y) / den;
    for i in 0..=den as i32
    {
        let n = min_x + i as f64 * den_x_range;
        let modified:Vec<String> = func.iter().map(|i| i.replace('x', &(n).to_string())).collect();
        for g in 0..=den as i32
        {
            let f = min_y + g as f64 * den_y_range;
            let num = match do_math(modified.iter().map(|j| j.replace('y', &(f).to_string())).collect())
            {
                Ok(n) => n,
                Err(_) =>
                {
                    println!("0");
                    continue;
                }
            };
            let (a, b) = parse(&num);
            let a = (a * 1e12).round() / 1e12;
            let b = (b * 1e12).round() / 1e12;
            re.push([n, f, a]);
            im.push([n, f, b]);
        }
    }
    (re, im)
}
fn get_list_2d(func:&[String], range:([[f64; 2]; 3], f64, f64)) -> (Vec<[f64; 2]>, Vec<[f64; 2]>)
{
    let mut re = Vec::new();
    let mut im = Vec::new();
    let min = range.0[0][0];
    let max = range.0[0][1];
    let den = range.1;
    let den_range = (max - min) / den;
    for i in 0..=den as i32
    {
        let n = min + i as f64 * den_range;
        let num = match do_math(func.iter().map(|i| i.replace('x', &(n).to_string())).collect())
        {
            Ok(n) => n,
            Err(_) =>
            {
                println!("0");
                continue;
            }
        };
        let (a, b) = parse(&num);
        let a = (a * 1e12).round() / 1e12;
        let b = (b * 1e12).round() / 1e12;
        re.push([n, a]);
        im.push([n, b]);
    }
    (re, im)
}
fn graph(func:&[String], graph:bool, im3d:bool, re3d:bool, fg:&mut Figure, older:Option<Vec<[Vec<[f64; 2]>; 2]>>, range:([[f64; 2]; 3], f64, f64)) -> Option<[Vec<[f64; 2]>; 2]>
{
    fg.close();
    if graph
    {
        let (re, im) = get_list_3d(func, range);
        let i = im.iter().map(|i| i[2]).sum::<f64>() != 0.0;
        let r = re.iter().map(|i| i[2]).sum::<f64>() != 0.0;
        if re3d && im3d && i && r
        {
            fg.axes3d()
              .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
              .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
              .set_z_range(Fix(range.0[2][0]), Fix(range.0[2][1]))
              .points(re.iter().map(|i| i[0]), re.iter().map(|i| i[1]), re.iter().map(|i| i[2]), &[PointSymbol('.')])
              .points(im.iter().map(|i| i[0]), im.iter().map(|i| i[1]), im.iter().map(|i| i[2]), &[PointSymbol('.')]);
        }
        else if re3d && r
        {
            fg.axes3d().points(re.iter().map(|i| i[0]), re.iter().map(|i| i[1]), re.iter().map(|i| i[2]), &[PointSymbol('.')]);
        }
        else if im3d && i
        {
            fg.axes3d().points(im.iter().map(|i| i[0]), im.iter().map(|i| i[1]), im.iter().map(|i| i[2]), &[PointSymbol('.')]);
        }
        fg.show_and_keep_running().unwrap();
        return None;
    }
    let (re, im) = get_list_2d(func, range);
    if let Some(..) = older
    {
        let older = older.unwrap();
        if !older.is_empty()
        {
            let mut older_re = older[0][0].to_vec();
            let mut older_im = older[0][1].to_vec();
            for i in older
            {
                older_re.extend_from_slice(&i[0]);
                older_im.extend_from_slice(&i[1]);
            }
            fg.axes2d()
              .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
              .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
              .points(re.iter().map(|x| x[0]), re.iter().map(|x| x[1]), &[PointSymbol('.')])
              .points(im.iter().map(|x| x[0]), im.iter().map(|x| x[1]), &[PointSymbol('.')])
              .points(older_re.iter().map(|x| x[0]), older_re.iter().map(|x| x[1]), &[PointSymbol('.')])
              .points(older_im.iter().map(|x| x[0]), older_im.iter().map(|x| x[1]), &[PointSymbol('.')]);
            fg.show_and_keep_running().unwrap();
            return Some([re, im]);
        }
    }
    fg.axes2d()
      .set_y_range(Fix(range.0[1][0]), Fix(range.0[1][1]))
      .set_x_range(Fix(range.0[0][0]), Fix(range.0[0][1]))
      .points(re.iter().map(|x| x[0]), re.iter().map(|x| x[1]), &[PointSymbol('.')])
      .points(im.iter().map(|x| x[0]), im.iter().map(|x| x[1]), &[PointSymbol('.')]);
    fg.show_and_keep_running().unwrap();
    Some([re, im])
}