mod complex;
mod equal;
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
use graph::graph;
use print::{print_answer, print_concurrent};
#[cfg(target_os = "linux")]
use {
    libc::{isatty, STDIN_FILENO}, std::io::stdin
};
fn help()
{
    println!(
             "Usage: calc [FLAGS] function_1 function_2 function_3...\n\
FLAGS: --help (this message),--debug for computation time in nanoseconds\n\n\
- Type \"exit\" to exit the program.\n\
- Type \"clear\" to clear the screen.\n\
- Type \"help\" to see this message.\n\n\
Trigonometric functions:\n\
- sin, cos, tan, asin, acos, atan\n\
- csc, sec, cot, acsc, asec, acot\n\
- sinh, cosh, tanh, asinh, acosh, atanh\n\
- csch, sech, coth, acsch, asech, acoth\n\n\
Other functions:\n\
- sqrt, cbrt\n\
- ln, log(base,num), root(base,exp)\n\
- abs, sgn, arg\n\
- ceil, floor, round, int, frac\n\
- fact, subfact\n\
- sinc, exp\n\
- deg(to_degrees), rad(to_radians)\n\
- re(real part), im(imaginary part)\n\n\
Special features:\n\
- Graphing: type a function with one variable and add \"graphs\" to graph it.\n\
- Graphing multiple functions: use the \"#\" character to separate the functions.\n\
- Change the x range of the graph: use \"xr=min,max\".\n\
- Change the number of points in the graph: use \"2d=num_points\" for 2D graphs or \"3d=num_points\" for 3D graphs.\n\n\
Examples:\n\
- To calculate the sine of 0.5, type: sin(0.5)\n\
- To calculate the logarithm base 2 of 8, type: log(2,8)\n\
- To graph x^2, type: x^2 graphs\n\
- To graph x^2, x^3, and x, type: x^2#x^3#x graphs\n\
- To change the x range to -10 to 10, type: xr=-10,10\n\
- To change the number of points to 100000 for a 2D graph, type: 2d=100000"
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
    let mut range = ([[-10.0, 10.0]; 3], 40000.0, 400.0);
    let mut plot = Figure::new();
    plot.set_enhanced_text(false);
    let mut watch = None;
    let mut args = args().collect::<Vec<String>>();
    args.remove(0);
    let debug = !args.is_empty() && args[0] == "--debug";
    if debug
    {
        args.remove(0);
    }
    if !args.is_empty()
    {
        if debug
        {
            watch = Some(std::time::Instant::now());
        }
        if args[0] == "--help"
        {
            help();
            return;
        }
        for i in args
        {
            let input = i.replace('z', "(x+y*i)");
            if input.contains('=')
            {
                let mut split = input.split('=');
                let l = split.next().unwrap();
                let r = split.next().unwrap();
                if equal::equal(range, input.as_str(), l, r)
                {
                    return;
                }
            }
            let func = match get_func(input.as_str())
            {
                Ok(f) => f,
                Err(()) =>
                {
                    println!("Invalid function.");
                    return;
                }
            };
            if input.contains('x')
            {
                let mut split = input.split('#');
                let l = split.next().unwrap();
                let m = split.next().unwrap_or("0");
                let r = split.next().unwrap_or("0");
                let funcl = match get_func(l)
                {
                    Ok(f) => f,
                    _ =>
                    {
                        println!("Invalid function.");
                        return;
                    }
                };
                let funcm = match get_func(m)
                {
                    Ok(f) => f,
                    _ =>
                    {
                        println!("Invalid function.");
                        return;
                    }
                };
                let funcr = match get_func(r)
                {
                    Ok(f) => f,
                    _ =>
                    {
                        println!("Invalid function.");
                        return;
                    }
                };
                if input.contains('y')
                {
                    graph([&l.to_string(), &m.to_string(), &r.to_string()], [&funcl, &funcm, &funcr], true, true, &mut plot, range);
                    return;
                }
                graph([&l.to_string(), &m.to_string(), &r.to_string()], [&funcl, &funcm, &funcr], false, true, &mut plot, range);
                return;
            }
            print_answer(func);
        }
        if let Some(time) = watch
        {
            println!("{}", time.elapsed().as_nanos());
        }
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
        print_answer(match get_func(&input)
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
    let fg = "\x1b[96m";
    let mut lines:Vec<String>;
    let (mut c, mut i, mut max, mut cursor, mut frac, mut len, mut l, mut m, mut r, mut funcl, mut funcm, mut funcr, mut split);
    loop
    {
        input.clear();
        stdout().flush().unwrap();
        lines = BufReader::new(File::open(file_path).unwrap()).lines().map(|l| l.unwrap()).collect();
        i = lines.len() as i32;
        max = i;
        cursor = 0;
        frac = false;
        'outer: loop
        {
            c = read_single_char();
            if debug
            {
                watch = Some(std::time::Instant::now());
            }
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
                    frac = print_concurrent(&input, var.clone(), false);
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
                    len = input.len();
                    if let Some(time) = watch
                    {
                        let time = time.elapsed().as_nanos();
                        len += time.to_string().len() + 1;
                        print!(" {}", time);
                    }
                    for _ in 0..(len - cursor)
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
            l = input.split('=').next().unwrap();
            r = input.split('=').last().unwrap();
            match l
            {
                "xr" =>
                {
                    range.0[0][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                    range.0[0][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                    continue;
                }
                "yr" =>
                {
                    range.0[1][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                    range.0[1][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                    continue;
                }
                "zr" =>
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
            if equal::equal(range, &input, l, r)
            {
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
        else if (input.replace("exp", "").contains('x') && var.iter().all(|i| i[0] != 'x')) || (input.contains('z') && var.iter().all(|i| i[0] != 'z'))
        {
            write(&input, &mut file, &lines);
            input = input.replace('z', "(x+y*i)");
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            split = input.split('#');
            l = split.next().unwrap();
            m = split.next().unwrap_or("0");
            r = split.next().unwrap_or("0");
            funcl = match get_func(l)
            {
                Ok(f) => f,
                _ => continue,
            };
            funcm = match get_func(m)
            {
                Ok(f) => f,
                _ => continue,
            };
            funcr = match get_func(r)
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
            if c.is_ascii_alphanumeric()
               || c == '+'
               || c == '-'
               || c == '*'
               || c == '/'
               || c == '^'
               || c == '('
               || c == ')'
               || c == '.'
               || c == '='
               || c == ','
               || c == 'π'
               || c == '#'
               || c == '|'
               || c == '!'
               || c == '%'
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