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
use terminal_size::{Width, terminal_size};
#[cfg(target_os = "linux")]
use {
    libc::{isatty, STDIN_FILENO}, std::io::stdin
};
fn help()
{
    println!(
             "Usage: calc [FLAGS] function_1 function_2 function_3...\n\
FLAGS: --help (this message)\n\
--tau fractions are shown in tau instead of pi\n\
--deg compute in degrees, gets rid of complex support for non hyperbolic trig functions\n\
--2d [num] number of points to graph in 2D (default 40000)\n\
--3d [num] number of points to graph in 3D (default 400)\n\
--xr [min] [max] x range for graphing\n\
--yr [min] [max] y range for graphing\n\
--zr [min] [max] z range for graphing\n\
--debug displays computation time in nanoseconds\n\n\
- Type \"exit\" to exit the program\n\
- Type \"clear\" to clear the screen\n\
- Type \"help\" to see this message\n\
- Type \"history\" to see the history of calculations\n\
- Type \"deg\" to switch to degrees mode\n\
- Type \"rad\" to switch to radians mode\n\
- Type \"tau\" to show fractions in tau\n\
- Type \"pi\" to show fractions in pi\n\
- Type 2d=[num] to set the number of points in 2D graphs (default 40000)\n\
- Type 3d=[num] to set the number of points in 3D graphs (default 400)\n\
- Type xr=[min],[max] to set the x range for graphing\n\
- Type yr=[min],[max] to set the y range for graphing\n\
- Type zr=[min],[max] to set the z range for graphing\n\
- Type \"debug\" to toggle debug mode.\n\n\
Trigonometric functions:\n\
- sin, cos, tan, asin, acos, atan\n\
- csc, sec, cot, acsc, asec, acot\n\
- sinh, cosh, tanh, asinh, acosh, atanh\n\
- csch, sech, coth, acsch, asech, acoth\n\n\
Other functions:\n\
- sqrt, cbrt, square, cube\n\
- ln, log(base,num), root(base,exp)\n\
- abs, sgn, arg\n\
- ceil, floor, round, int, frac\n\
- fact, subfact\n\
- sinc, cis, exp\n\
- deg(to_degrees), rad(to_radians)\n\
- re(real part), im(imaginary part)"
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
    let mut debug = false;
    let mut tau = false;
    let mut deg = false;
    loop
    {
        if !args.is_empty()
        {
            match args[0].as_str()
            {
                "--debug" => debug = true,
                "--tau" => tau = true,
                "--deg" => deg = true,
                "--2d" =>
                {
                    if args.len() > 1
                    {
                        range.1 = args[1].parse::<f64>().unwrap_or(40000.0);
                        args.remove(0);
                    }
                }
                "--3d" =>
                {
                    if args.len() > 1
                    {
                        range.2 = args[1].parse::<f64>().unwrap_or(400.0);
                        args.remove(0);
                    }
                }
                "--yr" =>
                {
                    if args.len() > 2
                    {
                        range.0[1][0] = args[1].parse::<f64>().unwrap_or(-10.0);
                        range.0[1][1] = args[2].parse::<f64>().unwrap_or(10.0);
                        args.remove(0);
                        args.remove(0);
                    }
                }
                "--xr" =>
                {
                    if args.len() > 2
                    {
                        range.0[0][0] = args[1].parse::<f64>().unwrap_or(-10.0);
                        range.0[0][1] = args[2].parse::<f64>().unwrap_or(10.0);
                        args.remove(0);
                        args.remove(0);
                    }
                }
                "--zr" =>
                {
                    if args.len() > 2
                    {
                        range.0[2][0] = args[1].parse::<f64>().unwrap_or(-10.0);
                        range.0[2][1] = args[2].parse::<f64>().unwrap_or(10.0);
                        args.remove(0);
                        args.remove(0);
                    }
                }
                "--help" =>
                {
                    help();
                    return;
                }
                _ => break,
            }
            args.remove(0);
        }
        else
        {
            break;
        }
    }
    if !args.is_empty()
    {
        if debug
        {
            watch = Some(std::time::Instant::now());
        }
        for i in args
        {
            let input = i.replace('z', "(x+y*i)").replace(' ', "");
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
                    graph([&l.to_string(), &m.to_string(), &r.to_string()], [&funcl, &funcm, &funcr], true, true, &mut plot, range, deg);
                    return;
                }
                graph([&l.to_string(), &m.to_string(), &r.to_string()], [&funcl, &funcm, &funcr], false, true, &mut plot, range, deg);
                return;
            }
            print_answer(func, deg);
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
                     },
                     deg);
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
    let (mut width, mut c, mut i, mut max, mut cursor, mut frac, mut len, mut l, mut m, mut r, mut funcl, mut funcm, mut funcr, mut split);
    loop
    {
        width = match terminal_size()
        {
            Some((Width(w), _)) => w,
            _ => 80,
        };
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
                            print!("\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A");
                            stdout().flush().unwrap();
                        }
                        continue;
                    }
                    cursor -= 1;
                    input.remove(cursor);
                    print!("\x1B[2K\x1B[1G{}", input);
                    if width < (input.len() + 1) as u16
                    {
                        print!("\x1b[A");
                    }
                    if input.is_empty()
                    {
                        frac = print_concurrent(&"0".to_string(), var.clone(), true, tau, deg);
                    }
                    else
                    {
                        frac = print_concurrent(&input, var.clone(), false, tau, deg);
                    }
                    for _ in 0..(input.len() - cursor)
                    {
                        print!("\x08");
                    }
                    if cursor == 0 && input.is_empty()
                    {
                        print!("\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A");
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
                    if width < (input.len() + 1) as u16
                    {
                        print!("\x1b[A");
                    }
                    frac = print_concurrent(&input, var.clone(), false, tau, deg);
                    print!("\x1B[2K\x1B[1G{fg}{}\x1b[0m", input);
                }
                '\x1E' =>
                {
                    i += 1;
                    if i >= max
                    {
                        input.clear();
                        print!("\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A");
                        print!("\x1B[2K\x1B[1G{fg}\x1b[0m");
                        stdout().flush().unwrap();
                        i = max;
                        cursor = 0;
                        continue 'outer;
                    }
                    input = lines[i as usize].clone();
                    cursor = input.len();
                    if width < (input.len() + 1) as u16
                    {
                        print!("\x1b[A");
                    }
                    frac = print_concurrent(&input, var.clone(), false, tau, deg);
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
                    if c == 'τ'
                    {
                        input.insert(cursor, 't');
                        cursor += 1;
                        input.insert(cursor, 'a');
                        cursor += 1;
                        input.insert(cursor, 'u');
                        cursor += 1;
                    }
                    else
                    {
                        input.insert(cursor, c);
                        cursor += 1;
                    }
                    if width < (input.len() + 1) as u16
                    {
                        print!("\x1b[A");
                    }
                    frac = print_concurrent(&input, var.clone(), false, tau, deg);
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
        write(&input, &mut file, &lines);
        match input.as_str()
        {
            "deg" =>
            {
                deg = true;
                continue;
            }
            "rad" =>
            {
                deg = false;
                continue;
            }
            "tau" =>
            {
                tau = true;
                continue;
            }
            "pi" =>
            {
                tau = false;
                continue;
            }
            "clear" =>
            {
                print!("\x1B[2J\x1B[1;1H");
                stdout().flush().unwrap();
                continue;
            }
            "debug" =>
            {
                debug = !debug;
                watch = None;
                continue;
            }
            "help" =>
            {
                help();
                continue;
            }
            "history" =>
            {
                for l in lines
                {
                    println!("{}", l);
                }
                continue;
            }
            "exit" => break,
            _ =>
            {}
        }
        if input.is_empty()
        {
            continue;
        }
        if input.contains('=')
        {
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
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
                graph([&l.to_string(), &m.to_string(), &r.to_string()], [&funcl, &funcm, &funcr], true, false, &mut plot, range, deg);
                continue;
            }
            graph([&l.to_string(), &m.to_string(), &r.to_string()], [&funcl, &funcm, &funcr], false, false, &mut plot, range, deg);
            continue;
        }
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
               || c == 'τ'
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