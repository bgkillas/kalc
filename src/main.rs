mod complex;
mod equal;
mod fraction;
mod graph;
mod math;
mod parse;
mod print;
#[cfg(test)]
mod tests;
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
    libc::{isatty, STDIN_FILENO, ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ}, std::io::stdin
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
--point [char] point style for graphing\n\
--sci enables scientific notation\n\
--base [num] sets the number base (2,8,16) (default 10)\n\
--debug displays computation time in nanoseconds\n\n\
- Type \"exit\" to exit the program\n\
- Type \"clear\" to clear the screen\n\
- Type \"help\" to see this message\n\
- Type \"history\" to see the history of calculations\n\
- Type \"deg\" to switch to degrees mode\n\
- Type \"rad\" to switch to radians mode\n\
- Type \"tau\" to show fractions in tau\n\
- Type \"pi\" to show fractions in pi\n\
- Type \"2d=[num]\" to set the number of points in 2D graphs (default 40000)\n\
- Type \"3d=[num]\" to set the number of points in 3D graphs (default 400)\n\
- Type \"xr=[min],[max]\" to set the x range for graphing\n\
- Type \"yr=[min],[max]\" to set the y range for graphing\n\
- Type \"zr=[min],[max]\" to set the z range for graphing\n\
- Type \"point=[char]\" to set the point style for graphing\n\
- Type \"sci\" to toggle scientific notation\n\
- Type \"base=[num]\" to set the number base (2,8,16) (default 10)\n\
- Type \"debug\" toggles displaying computation time in nanoseconds\n\
- Type \"_\" to use the previous answer\n\n\
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
fn write(input:&str, file:&mut File, lines:&Vec<String>)
{
    if lines.is_empty() || lines[lines.len() - 1] != *input
    {
        file.write_all(input.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}
fn main()
{
    let mut graph_options = ([[-10.0, 10.0]; 3], 40000.0, 400.0, '.'); //[xr,yr,zr], 2d, 3d, point style
    let mut print_options = (false, false, 10); //[sci, deg, #base]
    let mut plot = Figure::new();
    plot.set_enhanced_text(false);
    let mut watch = None;
    let mut args = args().collect::<Vec<String>>();
    args.remove(0);
    let mut debug = false;
    let mut tau = false;
    while !args.is_empty()
    {
        match args[0].as_str()
        {
            "--debug" => debug = true,
            "--tau" => tau = true,
            "--deg" => print_options.1 = true,
            "--2d" =>
            {
                if args.len() > 1
                {
                    graph_options.1 = args[1].parse::<f64>().unwrap_or(40000.0);
                    args.remove(0);
                }
            }
            "--3d" =>
            {
                if args.len() > 1
                {
                    graph_options.2 = args[1].parse::<f64>().unwrap_or(400.0);
                    args.remove(0);
                }
            }
            "--yr" =>
            {
                if args.len() > 2
                {
                    graph_options.0[1][0] = args[1].parse::<f64>().unwrap_or(-10.0);
                    graph_options.0[1][1] = args[2].parse::<f64>().unwrap_or(10.0);
                    args.remove(0);
                    args.remove(0);
                }
            }
            "--xr" =>
            {
                if args.len() > 2
                {
                    graph_options.0[0][0] = args[1].parse::<f64>().unwrap_or(-10.0);
                    graph_options.0[0][1] = args[2].parse::<f64>().unwrap_or(10.0);
                    args.remove(0);
                    args.remove(0);
                }
            }
            "--zr" =>
            {
                if args.len() > 2
                {
                    graph_options.0[2][0] = args[1].parse::<f64>().unwrap_or(-10.0);
                    graph_options.0[2][1] = args[2].parse::<f64>().unwrap_or(10.0);
                    args.remove(0);
                    args.remove(0);
                }
            }
            "--base" =>
            {
                if args.len() > 1
                {
                    print_options.2 = args[1].parse::<usize>().unwrap_or(10);
                    args.remove(0);
                }
            }
            "--sci" => print_options.0 = true,
            "--point" =>
            {
                graph_options.3 = args[1].chars().next().unwrap_or('.');
                args.remove(0);
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
                if equal::equal(graph_options, input.as_str(), l, r)
                {
                    continue;
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
                    graph([l, m, r], [&funcl, &funcm, &funcr], true, true, &mut plot, graph_options, print_options.1);
                    continue;
                }
                graph([l, m, r], [&funcl, &funcm, &funcr], false, true, &mut plot, graph_options, print_options.1);
                continue;
            }
            print_answer(func, print_options);
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
                     print_options);
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
    let mut last = String::new();
    #[cfg(target_os = "linux")]
    let mut width;
    let (mut c, mut i, mut max, mut cursor, mut frac, mut len, mut l, mut m, mut r, mut funcl, mut funcm, mut funcr, mut split);
    loop
    {
        #[cfg(target_os = "linux")]
        {
            width = get_terminal_width();
        }
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
                    #[cfg(target_os = "linux")]
                    if width < (input.len() + 1) as u16
                    {
                        print!("\x1b[A");
                    }
                    if input.is_empty()
                    {
                        frac = print_concurrent("0", var.clone(), true, tau, &last, print_options);
                    }
                    else
                    {
                        frac = print_concurrent(&input, var.clone(), false, tau, &last, print_options);
                    }
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
                    #[cfg(target_os = "linux")]
                    if width < (input.len() + 1) as u16
                    {
                        print!("\x1b[A");
                    }
                    frac = print_concurrent(&input, var.clone(), false, tau, &last, print_options);
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
                    #[cfg(target_os = "linux")]
                    if width < (input.len() + 1) as u16
                    {
                        print!("\x1b[A");
                    }
                    frac = print_concurrent(&input, var.clone(), false, tau, &last, print_options);
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
                    else if c == 'τ'
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
                    #[cfg(target_os = "linux")]
                    if width < (input.len() + 1) as u16
                    {
                        print!("\x1b[A");
                    }
                    frac = print_concurrent(&input, var.clone(), false, tau, &last, print_options);
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
        if input.is_empty()
        {
            continue;
        }
        match input.as_str()
        {
            "deg" =>
            {
                print_options.1 = true;
                continue;
            }
            "rad" =>
            {
                print_options.1 = false;
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
            "sci" =>
            {
                print_options.0 = true;
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
        if input.len() > 6 && &input[0..6] == "point="
        {
            graph_options.3 = input[6..].chars().next().unwrap();
            continue;
        }
        if input.len() > 5 && &input[0..5] == "base="
        {
            print_options.2 = input[5..].parse::<usize>().unwrap();
            continue;
        }
        last = format!("({})", input.clone());
        write(&input, &mut file, &lines);
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
                    graph_options.0[0][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                    graph_options.0[0][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                    continue;
                }
                "yr" =>
                {
                    graph_options.0[1][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                    graph_options.0[1][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                    continue;
                }
                "zr" =>
                {
                    graph_options.0[2][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                    graph_options.0[2][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                    continue;
                }
                "2d" =>
                {
                    graph_options.1 = r.parse::<f64>().unwrap();
                    continue;
                }
                "3d" =>
                {
                    graph_options.2 = r.parse::<f64>().unwrap();
                    continue;
                }
                _ => (),
            }
            if equal::equal(graph_options, &input, l, r)
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
                graph([l, m, r], [&funcl, &funcm, &funcr], true, false, &mut plot, graph_options, print_options.1);
                continue;
            }
            graph([l, m, r], [&funcl, &funcm, &funcr], false, false, &mut plot, graph_options, print_options.1);
            continue;
        }
        println!();
    }
}
#[cfg(target_os = "linux")]
fn get_terminal_width() -> u16
{
    unsafe {
        let mut size:winsize = std::mem::zeroed();
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) == 0
        {
            size.ws_col
        }
        else
        {
            80
        }
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
               || c == '_'
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