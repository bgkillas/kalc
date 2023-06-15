mod fraction;
mod graph;
mod math;
mod options;
mod parse;
mod print;
#[cfg(test)]
mod tests;
use parse::{get_func, get_vars, input_var};
use std::env::{args, var};
use std::io::{BufRead, BufReader, IsTerminal, stdout, Write};
use std::fs::{File, OpenOptions};
use graph::graph;
use print::{print_answer, print_concurrent};
use std::io::stdin;
use std::thread::JoinHandle;
use options::{arg_opts, file_opts};
#[cfg(not(unix))]
use console::{Key, Term};
#[cfg(unix)]
use {
    libc::{ECHO, ICANON, tcgetattr, TCSANOW, tcsetattr, VMIN, VTIME}, std::io::Read, std::os::fd::AsRawFd
};
// gui support
// support unit conversions
// support vector operations
// allow units to be used in the input, and be outputted
fn main()
{
    let mut graph_options = GraphOptions::default();
    let mut print_options = PrintOptions::default();
    let mut watch = None;
    let mut allow_vars = true;
    let mut debug = false;
    let mut prec = 256;
    let mut handles:Vec<JoinHandle<()>> = Vec::new();
    #[cfg(unix)]
    let file_path = &(var("HOME").unwrap() + "/.config/kalc.config");
    #[cfg(not(unix))]
    let file_path = &format!("C:\\Users\\{}\\AppData\\Roaming\\kalc.config", var("USERNAME").unwrap());
    let mut args = args().collect::<Vec<String>>();
    if file_opts(&mut graph_options, &mut print_options, &mut allow_vars, &mut debug, &mut prec, file_path)
       || arg_opts(&mut graph_options, &mut print_options, &mut allow_vars, &mut debug, &mut prec, &mut args)
    {
        std::process::exit(1);
    }
    let mut vars:Vec<[String; 2]> = if allow_vars { get_vars(prec) } else { Vec::new() };
    #[cfg(unix)]
    let file_path = &(var("HOME").unwrap() + "/.config/kalc.vars");
    #[cfg(not(unix))]
    let file_path = &format!("C:\\Users\\{}\\AppData\\Roaming\\kalc.vars", var("USERNAME").unwrap());
    if File::open(file_path).is_ok() && allow_vars
    {
        let lines = BufReader::new(File::open(file_path).unwrap()).lines().map(|l| l.unwrap()).collect::<Vec<String>>();
        let mut split;
        for i in lines
        {
            split = i.split('=');
            vars.push([split.next().unwrap().to_string(), split.next().unwrap().to_string()]);
        }
    }
    let mut input = String::new();
    if !stdin().is_terminal()
    {
        for line in stdin().lock().lines()
        {
            if !line.as_ref().unwrap().is_empty()
            {
                args.push(line.unwrap());
            }
        }
    }
    #[cfg(unix)]
    let file_path = &(var("HOME").unwrap() + "/.config/kalc.history");
    #[cfg(not(unix))]
    let file_path = &format!("C:\\Users\\{}\\AppData\\Roaming\\kalc.history", var("USERNAME").unwrap());
    if File::open(file_path).is_err()
    {
        File::create(file_path).unwrap();
    }
    let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
    let mut lines:Vec<String>;
    let mut unmod_lines:Vec<String>;
    let mut last = String::new();
    let mut current = String::new();
    let mut inputs:Vec<String>;
    let (mut c, mut i, mut max, mut cursor, mut frac, mut len, mut l, mut r, mut split, mut funcs, mut v);
    let mut exit = false;
    'main: loop
    {
        if exit
        {
            for handle in handles
            {
                handle.join().unwrap();
            }
            break;
        }
        input.clear();
        frac = false;
        if !args.is_empty()
        {
            if debug
            {
                watch = Some(std::time::Instant::now());
            }
            input = args.first().unwrap().replace(' ', "").replace('_', &format!("({})", last));
            args.remove(0);
            print_answer(&input,
                         match get_func(&input_var(&input.replace('π', "pi").replace('τ', "tau"), &vars), prec)
                         {
                             Ok(f) => f,
                             Err(()) =>
                             {
                                 println!("Invalid function.");
                                 return;
                             }
                         },
                         print_options,
                         prec);
            if let Some(time) = watch
            {
                print!(" {}", time.elapsed().as_nanos());
            }
            if !(input.is_empty()
                 || (input.contains('x') && !input.contains("exp") && vars.iter().all(|i| i[0] != "x"))
                 || (input.contains('y') && vars.iter().all(|i| i[0] != "y"))
                 || (input.contains('z') && !input.contains("zeta") && vars.iter().all(|i| i[0] != "z"))
                 || (input.contains('=') && !(input.contains("!=") || input.contains("==") || input.contains(">=") || input.contains("<="))))
            {
                println!();
            }
            last = input.clone();
            if args.is_empty()
            {
                exit = true;
            }
        }
        else
        {
            if print_options.prompt
            {
                print!("{}> \x1b[0m", if print_options.color { "\x1b[94m" } else { "" });
                stdout().flush().unwrap();
            }
            current.clear();
            lines = BufReader::new(File::open(file_path).unwrap()).lines().map(|l| l.unwrap()).collect();
            unmod_lines = lines.clone();
            i = lines.len() as i32;
            max = i;
            cursor = 0;
            last = lines.last().unwrap_or(&String::new()).clone();
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
                        if !print_options.real_time_output
                        {
                            frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), print_options, prec);
                        }
                        if frac
                        {
                            println!();
                        }
                        if !(input.is_empty()
                             || (input.contains('x') && !input.contains("exp") && vars.iter().all(|i| i[0] != "x"))
                             || (input.contains('y') && vars.iter().all(|i| i[0] != "y"))
                             || (input.contains('z') && !input.contains("zeta") && vars.iter().all(|i| i[0] != "z"))
                             || (input.contains('=') && !(input.contains("!=") || input.contains("==") || input.contains(">=") || input.contains("<="))))
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
                                print!("\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A{}",
                                       if print_options.prompt
                                       {
                                           if print_options.color
                                           {
                                               "\x1b[94m> \x1b[0m"
                                           }
                                           else
                                           {
                                               "> "
                                           }
                                       }
                                       else
                                       {
                                           ""
                                       });
                                stdout().flush().unwrap();
                            }
                            continue;
                        }
                        cursor -= 1;
                        input.remove(cursor);
                        if i == max
                        {
                            current = input.clone();
                        }
                        else
                        {
                            lines[i as usize] = input.clone();
                        }
                        print!("\x1B[2K\x1B[1G{}", input);
                        frac = if input.is_empty()
                        {
                            false
                        }
                        else if print_options.real_time_output
                        {
                            print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), print_options, prec)
                        }
                        else
                        {
                            print!("\x1B[2K\x1B[1G{}{}\x1b[0m",
                                   if print_options.prompt
                                   {
                                       if print_options.color
                                       {
                                           "\x1b[94m> \x1b[96m"
                                       }
                                       else
                                       {
                                           "> "
                                       }
                                   }
                                   else if print_options.color
                                   {
                                       "\x1b[96m"
                                   }
                                   else
                                   {
                                       ""
                                   },
                                   input);
                            false
                        };
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
                            print!("\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A{}",
                                   if print_options.prompt
                                   {
                                       if print_options.color
                                       {
                                           "\x1b[94m> \x1b[0m"
                                       }
                                       else
                                       {
                                           "> "
                                       }
                                   }
                                   else
                                   {
                                       ""
                                   });
                        }
                    }
                    '\x1D' =>
                    {
                        // up history
                        i -= if i > 0 { 1 } else { 0 };
                        input = lines[i as usize].clone();
                        cursor = input.len();
                        if print_options.real_time_output
                        {
                            frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), print_options, prec);
                        }
                        print!("\x1B[2K\x1B[1G{}{}\x1b[0m",
                               if print_options.prompt
                               {
                                   if print_options.color
                                   {
                                       "\x1b[94m> \x1b[96m"
                                   }
                                   else
                                   {
                                       "> "
                                   }
                               }
                               else if print_options.color
                               {
                                   "\x1b[96m"
                               }
                               else
                               {
                                   ""
                               },
                               input);
                    }
                    '\x1E' =>
                    {
                        // down history
                        i += 1;
                        if i >= max
                        {
                            input = current.clone();
                            i = max;
                            if input.is_empty()
                            {
                                print!("\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A\x1B[2K\x1B[1G{}",
                                       if print_options.prompt
                                       {
                                           if print_options.color
                                           {
                                               "\x1b[94m> \x1b[0m"
                                           }
                                           else
                                           {
                                               "> "
                                           }
                                       }
                                       else
                                       {
                                           ""
                                       });
                                stdout().flush().unwrap();
                                cursor = 0;
                                continue 'outer;
                            }
                        }
                        else
                        {
                            input = lines[i as usize].clone();
                        }
                        cursor = input.len();
                        if print_options.real_time_output
                        {
                            frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), print_options, prec);
                        }
                        print!("\x1B[2K\x1B[1G{}{}\x1b[0m",
                               if print_options.prompt
                               {
                                   if print_options.color
                                   {
                                       "\x1b[94m> \x1b[96m"
                                   }
                                   else
                                   {
                                       "> "
                                   }
                               }
                               else if print_options.color
                               {
                                   "\x1b[96m"
                               }
                               else
                               {
                                   ""
                               },
                               input);
                    }
                    '\x1B' =>
                    {
                        // go left
                        if cursor == 0
                        {
                            continue;
                        }
                        cursor -= 1;
                        print!("\x08");
                    }
                    '\x1C' =>
                    {
                        // go right
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
                        if i == max
                        {
                            current = input.clone();
                        }
                        else
                        {
                            lines[i as usize] = input.clone();
                        }
                        if print_options.real_time_output
                        {
                            frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), print_options, prec);
                        }
                        else
                        {
                            print!("\x1B[2K\x1B[1G{}{}\x1b[0m",
                                   if print_options.prompt
                                   {
                                       if print_options.color
                                       {
                                           "\x1b[94m> \x1b[96m"
                                       }
                                       else
                                       {
                                           "> "
                                       }
                                   }
                                   else if print_options.color
                                   {
                                       "\x1b[96m"
                                   }
                                   else
                                   {
                                       ""
                                   },
                                   input);
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
                    }
                }
                stdout().flush().unwrap();
            }
            if input.is_empty() || input.chars().filter(|c| *c == '=').count() > 2
            {
                continue;
            }
            match input.as_str()
            {
                "color" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.color = !print_options.color;
                    continue;
                }
                "prompt" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.prompt = !print_options.prompt;
                    continue;
                }
                "deg" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.deg = true;
                    continue;
                }
                "rad" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.deg = false;
                    continue;
                }
                "rt" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.real_time_output = !print_options.real_time_output;
                    continue;
                }
                "tau" =>
                {
                    print_options.tau = true;
                    write(&input, &mut file, &unmod_lines);
                    continue;
                }
                "pi" =>
                {
                    print_options.tau = false;
                    write(&input, &mut file, &unmod_lines);
                    continue;
                }
                "sci" | "scientific" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.sci = !print_options.sci;
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
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    debug = !debug;
                    watch = None;
                    continue;
                }
                "help" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    help();
                    continue;
                }
                "line" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    graph_options.lines = !graph_options.lines;
                    continue;
                }
                "comma" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.comma = !print_options.comma;
                    continue;
                }
                "history" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    for l in lines
                    {
                        println!("{}", l);
                    }
                    continue;
                }
                "vars" | "variables" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    for v in vars.iter()
                    {
                        println!("{}={}", v[0], v[1]);
                    }
                    continue;
                }
                "version" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                    continue;
                }
                "exit" | "quit" =>
                {
                    break;
                }
                _ =>
                {}
            }
            write(&input, &mut file, &unmod_lines);
        }
        if input.contains('=') && !(input.contains("!=") || input.contains("==") || input.contains(">=") || input.contains("<="))
        {
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            split = input.split('=');
            l = split.next().unwrap();
            r = split.next().unwrap();
            if l.is_empty()
            {
                continue;
            }
            match l
            {
                "point" =>
                {
                    if r == "." || r == "+" || r == "x" || r == "*" || r == "s" || r == "S" || r == "o" || r == "O" || r == "t" || r == "T" || r == "d" || r == "D" || r == "r" || r == "R"
                    {
                        graph_options.point_style = r.chars().next().unwrap();
                    }
                    else
                    {
                        println!("Invalid point type");
                    }
                    continue;
                }
                "base" =>
                {
                    print_options.base = match r.parse::<usize>()
                    {
                        Ok(n) =>
                        {
                            if !(2..=36).contains(&n)
                            {
                                println!("Invalid base");
                                print_options.base
                            }
                            else
                            {
                                n
                            }
                        }
                        Err(_) =>
                        {
                            println!("Invalid base");
                            print_options.base
                        }
                    };
                    continue;
                }
                "decimal" | "deci" | "decimals" =>
                {
                    print_options.decimal_places = match r.parse::<usize>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid decimal");
                            print_options.decimal_places
                        }
                    };
                    continue;
                }
                "prec" | "precision" =>
                {
                    prec = if r == "0"
                    {
                        println!("Invalid precision");
                        prec
                    }
                    else
                    {
                        match r.parse::<u32>()
                        {
                            Ok(n) => n,
                            Err(_) =>
                            {
                                println!("Invalid precision");
                                prec
                            }
                        }
                    };
                    if allow_vars
                    {
                        v = get_vars(prec);
                        vars = [v.clone(), vars[v.len()..].to_vec()].concat();
                    }
                    continue;
                }
                "xr" =>
                {
                    graph_options.xr[0] = match r.split(',').next().unwrap().parse::<f64>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            graph_options.xr[0]
                        }
                    };
                    graph_options.xr[1] = match r.split(',').last().unwrap().parse::<f64>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            graph_options.xr[1]
                        }
                    };
                    continue;
                }
                "yr" =>
                {
                    graph_options.yr[0] = match r.split(',').next().unwrap().parse::<f64>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            graph_options.yr[0]
                        }
                    };
                    graph_options.yr[1] = match r.split(',').last().unwrap().parse::<f64>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            graph_options.yr[1]
                        }
                    };
                    continue;
                }
                "zr" =>
                {
                    graph_options.zr[0] = match r.split(',').next().unwrap().parse::<f64>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            graph_options.zr[0]
                        }
                    };
                    graph_options.zr[1] = match r.split(',').last().unwrap().parse::<f64>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            graph_options.zr[1]
                        }
                    };
                    continue;
                }
                "2d" =>
                {
                    graph_options.samples_2d = match r.parse::<f64>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid 2d sample size");
                            graph_options.samples_2d
                        }
                    };
                    continue;
                }
                "3d" =>
                {
                    graph_options.samples_3d = match r.parse::<f64>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid 3d sample size");
                            graph_options.samples_3d
                        }
                    };
                    continue;
                }
                _ => (),
            }
            for (i, v) in vars.iter().enumerate()
            {
                if v[0].split('(').next() == l.split('(').next()
                {
                    if r.is_empty()
                    {
                        println!("{}", vars[i][1]);
                        stdout().flush().unwrap();
                    }
                    if r == "null"
                    {
                        vars.remove(i);
                    }
                    else
                    {
                        vars[i] = [l.to_string(), r.to_string()];
                    }
                    continue 'main;
                }
            }
            if r.is_empty()
            {
                println!("0");
                stdout().flush().unwrap();
            }
            vars.push([l.to_string(), r.to_string()]);
            continue;
        }
        else if (input.replace("exp", "").contains('x') && vars.iter().all(|i| i[0] != "x")) || (input.replace("zeta", "").contains('z') && vars.iter().all(|i| i[0] != "z"))
        {
            input = input.replace("zeta", "##ta##").replace('z', "(x+y*i)").replace("##ta##", "zeta");
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            inputs = input.split('#').map(String::from).collect();
            funcs = Vec::new();
            for i in &inputs
            {
                funcs.push(match get_func(&input_var(i, &vars), prec)
                     {
                         Ok(f) => f,
                         _ => continue 'main,
                     });
            }
            handles.push(graph(inputs, funcs, graph_options, print_options.deg, prec, watch));
            continue;
        }
    }
}
// #[cfg(unix)]
// fn get_terminal_width() -> u16
// {
//     unsafe {
//         let mut size:winsize = std::mem::zeroed();
//         if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) == 0
//         {
//             size.ws_col
//         }
//         else
//         {
//             80
//         }
//     }
// }
#[cfg(unix)]
fn read_single_char() -> char
{
    let stdin_fd = stdin().as_raw_fd();
    let orig_termios = unsafe {
        let mut termios = std::mem::zeroed();
        tcgetattr(stdin_fd, &mut termios);
        termios
    };
    let mut new_termios = orig_termios;
    new_termios.c_lflag &= !(ICANON | ECHO);
    new_termios.c_cc[VMIN] = 1;
    new_termios.c_cc[VTIME] = 0;
    unsafe {
        tcsetattr(stdin_fd, TCSANOW, &new_termios);
    }
    let mut input = [0u8; 1];
    stdin().read_exact(&mut input).unwrap();
    unsafe {
        tcsetattr(stdin_fd, TCSANOW, &orig_termios);
    }
    match input[0]
    {
        27 =>
        {
            let mut seq = [0u8; 1];
            stdin().read_exact(&mut seq).unwrap();
            if seq[0] != 91
            {
                return seq[0] as char;
            }
            stdin().read_exact(&mut seq).unwrap();
            match seq[0]
            {
                65 => '\x1D', // Up arrow key
                66 => '\x1E', // Down arrow key
                67 => '\x1C', // Right arrow key
                68 => '\x1B', // Left arrow key
                _ => read_single_char(),
            }
        }
        207 =>
        {
            let mut seq = [0u8; 1];
            stdin().read_exact(&mut seq).unwrap();
            match seq[0]
            {
                128 => 'π',
                132 => 'τ',
                _ => read_single_char(),
            }
        }
        127 => '\x08',
        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'+' | b'-' | b'*' | b'/' | b'^' | b'(' | b')' | b'.' | b'=' | b',' | b'#' | b'|' | b'&' | b'!' | b'%' | b'_' | b'<' | b'>' | b' ' | b'\n' =>
        {
            input[0] as char
        }
        _ => read_single_char(),
    }
}
#[cfg(not(unix))]
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
               || c == '&'
               || c == '!'
               || c == '%'
               || c == '_'
               || c == '<'
               || c == '>'
               || c == ' '
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
fn write(input:&str, file:&mut File, lines:&Vec<String>)
{
    if lines.is_empty() || lines[lines.len() - 1] != *input
    {
        file.write_all(input.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}
fn help()
{
    println!(
             "Usage: kalc [FLAGS] function_1 function_2 function_3...\n\
FLAGS: --help (this message)\n\
--PrintOptions.3 fractions are shown in PrintOptions.3 instead of pi\n\
--deg compute in degrees, gets rid of complex support for non hyperbolic trig functions\n\
--2d [num] number of points to graph in 2D\n\
--3d [num] number of points to graph in 3D\n\
--xr [min] [max] x range for graphing\n\
--yr [min] [max] y range for graphing\n\
--zr [min] [max] z range for graphing\n\
--point [char] point style for graphing\n\
--sci toggles scientific notation\n\
--base [num] sets the number base (2,8,16)\n\
--prompt toggles the prompt\n\
--color toggles color\n\
--comma toggles comma seperation\n\
--vars toggles default variables\n\
--line toggles line graphing\n\
--rt toggles real time printing\n\
--prec [num] sets the precision\n\
--deci [num] sets how many decimals to display, also max length of numerator and denominator in fractions\n\
--def ignores config file\n\
--debug displays computation time in nanoseconds\n\n\
- Type \"exit\" to exit the program\n\
- Type \"clear\" to clear the screen\n\
- Type \"help\" to see this message\n\
- Type \"history\" to see the history of calculations\n\
- Type \"deg\" to switch to degrees mode\n\
- Type \"rad\" to switch to radians mode\n\
- Type \"tau\" to show fractions in tau\n\
- Type \"pi\" to show fractions in pi\n\
- Type \"prompt\" to toggle the prompt\n\
- Type \"line\" to toggle line graphing\n\
- Type \"rt\" to toggle real time printing\n\
- Type \"color\" to toggle color\n\
- Type \"comma\" to toggle comma seperation\n\
- Type \"2d=[num]\" to set the number of points in 2D graphs\n\
- Type \"3d=[num]\" to set the number of points in 3D graphs\n\
- Type \"xr=[min],[max]\" to set the x range for graphing\n\
- Type \"yr=[min],[max]\" to set the y range for graphing\n\
- Type \"zr=[min],[max]\" to set the z range for graphing\n\
- Type \"prec=[num]\" to set the precision\n\
- Type \"deci=[num]\" to set how many decimals to display, also max length of numerator and denominator in fractions\n\
- Type \"point=[char]\" to set the point style for graphing\n\
- Type \"sci\" to toggle scientific notation\n\
- Type \"vars\" to list all variables\n\
- Type \"base=[num]\" to set the number base (2-36)\n\
- Type \"_\" to use the previous answer\n\
- Type \"a=[num]\" to define a variable\n\
- Type \"f(x)=...\" to define a function\n\
- Type \"f(x,y)=...\" to define a 2 variable function\n\
- Type \"f(x,y,z...)=...\" to define a multi variable function\n\
- Type \"f...=\" to display the definition of a function or variable\n\
- Type \"f...=null\" to delete a function or variable\n\
- Type \"debug\" toggles displaying computation time in nanoseconds\n\n\
Operators:\n\
- +, -, *, /, ^, %, <, >, <=, >=\n\
- !x (subfact), x! (fact)\n\
- && (and), || (or), == (equals), != (not equals)\n\
- >> (right shift), << (left shift)\n\n\
Trigonometric functions:\n\
- sin, cos, tan, asin, acos, atan\n\
- csc, sec, cot, acsc, asec, acot\n\
- sinh, cosh, tanh, asinh, acosh, atanh\n\
- csch, sech, coth, acsch, asech, acoth\n\n\
Other functions:\n\
- sqrt, cbrt, square, cube\n\
- ln, log(base,num), root(base,exp), sum(func,var,start,end), prod(func,var,start,end)\n\
- abs, sgn, arg\n\
- ceil, floor, round, int, frac\n\
- fact, subfact\n\
- sinc, cis, exp\n\
- zeta, gamma, erf, erfc, digamma, ai\n\
- deg(to_degrees), rad(to_radians)\n\
- re(real part), im(imaginary part)\n\n\
Constants:\n\
- c: speed of light, 299792458 m/s\n\
- g: gravity, 9.80665 m/s^2\n\
- G: gravitational constant, 6.67430E-11 m^3/(kg*s^2)\n\
- h: planck's constant, 6.62607015E-34 J*s\n\
- ec: elementary charge, 1.602176634E-19 C\n\
- me: electron mass, 9.1093837015E-31 kg\n\
- mp: proton mass, 1.67262192369E-27 kg\n\
- mn: neutron mass, 1.67492749804E-27 kg\n\
- ev: electron volt, 1.602176634E-19 J\n\
- kc: coulomb's constant, 8.9875517923E9 N*m^2/C^2\n\
- na: avogadro's number, 6.02214076E23 1/mol\n\
- r: gas constant, 8.31446261815324 J/(mol*K)\n\
- kb: boltzmann constant, 1.380649E-23 J/K\n\
- phi: golden ratio, 1.6180339887~\n\
- e: euler's number, 2.7182818284~\n\
- pi: pi, 3.1415926535~\n\
- tau: tau, 6.2831853071~"
    );
}
#[derive(Clone, Copy)]
pub struct GraphOptions
{
    xr:[f64; 2],
    yr:[f64; 2],
    zr:[f64; 2],
    samples_2d:f64,
    samples_3d:f64,
    point_style:char,
    lines:bool,
}
#[derive(Clone, Copy)]
pub struct PrintOptions
{
    sci:bool,
    deg:bool,
    base:usize,
    tau:bool,
    real_time_output:bool,
    decimal_places:usize,
    color:bool,
    prompt:bool,
    comma:bool,
}
impl Default for PrintOptions
{
    fn default() -> Self
    {
        PrintOptions { sci:false,
                       deg:false,
                       base:10,
                       tau:false,
                       real_time_output:true,
                       decimal_places:12,
                       color:false,
                       prompt:false,
                       comma:false }
    }
}
impl Default for GraphOptions
{
    fn default() -> Self
    {
        GraphOptions { xr:[-10.0, 10.0],
                       yr:[-10.0, 10.0],
                       zr:[-10.0, 10.0],
                       samples_2d:40000.0,
                       samples_3d:400.0,
                       point_style:'.',
                       lines:false }
    }
}