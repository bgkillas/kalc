mod fraction;
mod graph;
mod math;
mod parse;
mod print;
#[cfg(test)]
mod tests;
use parse::{input_var, get_func, get_vars};
use std::env::{args, var};
use std::io::{BufRead, BufReader, IsTerminal, stdout, Write};
use std::fs::{File, OpenOptions};
use graph::graph;
use print::{print_answer, print_concurrent};
use std::io::stdin;
use std::thread::JoinHandle;
#[cfg(not(unix))]
use console::{Key, Term};
#[cfg(unix)]
use {
    libc::{tcgetattr, ECHO, ICANON, TCSANOW, VMIN, VTIME, tcsetattr}, std::os::fd::AsRawFd, std::io::Read
};
fn main()
{
    let mut graph_options = ([[-10.0, 10.0]; 3], 40000.0, 400.0, '.', false); //[xr,yr,zr], 2d, 3d, point style, lines
    let mut watch = None;
    let mut print_options = (false, false, 10, false, true, 12); //[sci, deg, #base, tau, concurrent_output, decimal_places]
    let mut allow_vars = true;
    let mut debug = false;
    let mut prompt = true;
    let mut color = true;
    let mut prec = 256;
    let mut handles:Vec<JoinHandle<()>> = Vec::new();
    #[cfg(unix)]
    let file_path = &(var("HOME").unwrap() + "/.config/kalc.config");
    #[cfg(not(unix))]
    let file_path = &format!("C:\\Users\\{}\\AppData\\Roaming\\kalc.config", var("USERNAME").unwrap());
    if File::open(file_path).is_ok()
    {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let mut split;
        for line in reader.lines().map(|l| l.unwrap())
        {
            split = line.split('=');
            match split.next().unwrap()
            {
                "2d" => graph_options.1 = split.next().unwrap().parse::<f64>().unwrap_or(40000.0),
                "3d" => graph_options.2 = split.next().unwrap().parse::<f64>().unwrap_or(400.0),
                "xr" =>
                {
                    let mut xr = split.next().unwrap().split(',');
                    graph_options.0[0][0] = xr.next().unwrap().parse::<f64>().unwrap_or(-10.0);
                    graph_options.0[0][1] = xr.next().unwrap().parse::<f64>().unwrap_or(10.0);
                }
                "yr" =>
                {
                    let mut yr = split.next().unwrap().split(',');
                    graph_options.0[1][0] = yr.next().unwrap().parse::<f64>().unwrap_or(-10.0);
                    graph_options.0[1][1] = yr.next().unwrap().parse::<f64>().unwrap_or(10.0);
                }
                "zr" =>
                {
                    let mut zr = split.next().unwrap().split(',');
                    graph_options.0[2][0] = zr.next().unwrap().parse::<f64>().unwrap_or(-10.0);
                    graph_options.0[2][1] = zr.next().unwrap().parse::<f64>().unwrap_or(10.0);
                }
                "prec" | "precision" =>
                {
                    prec = {
                        let prec = split.next().unwrap().parse::<u32>().unwrap_or(256);
                        if prec == 0
                        {
                            256
                        }
                        else
                        {
                            prec
                        }
                    }
                }
                "decimal" | "deci" | "decimals" => print_options.5 = split.next().unwrap().parse::<usize>().unwrap_or(12),
                "rt" => print_options.4 = split.next().unwrap().parse::<bool>().unwrap_or(true),
                "line" => graph_options.4 = split.next().unwrap().parse::<bool>().unwrap_or(false),
                "prompt" => prompt = split.next().unwrap().parse::<bool>().unwrap_or(true),
                "color" => color = split.next().unwrap().parse::<bool>().unwrap_or(true),
                "point" => graph_options.3 = split.next().unwrap().chars().next().unwrap_or('.'),
                "sci" | "scientific" => print_options.0 = split.next().unwrap().parse::<bool>().unwrap_or(false),
                "base" =>
                {
                    print_options.2 = split.next().unwrap().parse::<usize>().unwrap_or(10);
                    if print_options.2 > 36 || print_options.2 < 2
                    {
                        print_options.2 = 10;
                    }
                }
                "debug" => debug = split.next().unwrap().parse::<bool>().unwrap_or(false),
                "deg" => print_options.1 = split.next().unwrap().parse::<bool>().unwrap_or(false),
                "tau" => print_options.3 = split.next().unwrap().parse::<bool>().unwrap_or(false),
                "vars" => allow_vars = split.next().unwrap().parse::<bool>().unwrap_or(true),
                _ =>
                {}
            }
        }
    }
    let mut args = args().collect::<Vec<String>>();
    args.remove(0);
    let (mut split, mut l);
    while !args.is_empty()
    {
        if args[0].contains('=') || args[0].contains(',')
        {
            l = args[0].clone();
            split = l.split(|c| c == '=' || c == ',');
            args[0] = split.next().unwrap().to_string();
            args.insert(1, split.next().unwrap().to_string());
            if split.clone().count() > 0
            {
                args.insert(2, split.next().unwrap().to_string());
            }
        }
        match args[0].as_str()
        {
            "--debug" => debug = !debug,
            "--tau" => print_options.3 = !print_options.3,
            "--deg" => print_options.1 = !print_options.1,
            "--prompt" => prompt = !prompt,
            "--color" => color = !color,
            "--line" => graph_options.4 = !graph_options.4,
            "--rt" => print_options.4 = !print_options.4,
            "--prec" | "--precision" =>
            {
                if args.len() > 1
                {
                    prec = if args[1] == "0" { 256 } else { args[1].parse::<u32>().unwrap_or(256) };
                    args.remove(0);
                }
            }
            "--decimal" | "--deci" | "--decimals" =>
            {
                if args.len() > 1
                {
                    print_options.5 = args[1].parse::<usize>().unwrap_or(12);
                    args.remove(0);
                }
            }
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
                    if print_options.2 > 36 || print_options.2 < 2
                    {
                        print_options.2 = 10;
                    }
                    args.remove(0);
                }
            }
            "--sci" | "--scientific" => print_options.0 = !print_options.0,
            "--point" =>
            {
                graph_options.3 = args[1].chars().next().unwrap_or('.');
                args.remove(0);
            }
            "--help" | "-h" =>
            {
                help();
                return;
            }
            "--version" | "-v" =>
            {
                println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                return;
            }
            "--vars" => allow_vars = !allow_vars,
            "--default" | "--def" =>
            {
                print_options = (false, false, 10, false, true, 12);
                graph_options = ([[-10.0, 10.0]; 3], 40000.0, 400.0, '.', false);
                prec = 256;
                allow_vars = true;
                debug = false;
                prompt = true;
                color = true;
            }
            _ => break,
        }
        args.remove(0);
    }
    let mut vars:Vec<[String; 2]> = get_vars(allow_vars, prec);
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
                         match get_func(&input_var(&input, &vars), prec)
                         {
                             Ok(f) => f,
                             Err(()) =>
                             {
                                 println!("Invalid function.");
                                 return;
                             }
                         },
                         print_options,
                         color,
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
            if prompt
            {
                print!("{}> \x1b[0m", if color { "\x1b[94m" } else { "" });
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
                        if !print_options.4
                        {
                            frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), print_options, prompt, color, prec);
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
                                       if prompt
                                       {
                                           if color
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
                        else if print_options.4
                        {
                            print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), print_options, prompt, color, prec)
                        }
                        else
                        {
                            print!("\x1B[2K\x1B[1G{}{}\x1b[0m",
                                   if prompt
                                   {
                                       if color
                                       {
                                           "\x1b[94m> \x1b[96m"
                                       }
                                       else
                                       {
                                           "> "
                                       }
                                   }
                                   else if color
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
                                   if prompt
                                   {
                                       if color
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
                        if print_options.4
                        {
                            frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), print_options, prompt, color, prec);
                        }
                        print!("\x1B[2K\x1B[1G{}{}\x1b[0m",
                               if prompt
                               {
                                   if color
                                   {
                                       "\x1b[94m> \x1b[96m"
                                   }
                                   else
                                   {
                                       "> "
                                   }
                               }
                               else if color
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
                                       if prompt
                                       {
                                           if color
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
                        if print_options.4
                        {
                            frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), print_options, prompt, color, prec);
                        }
                        print!("\x1B[2K\x1B[1G{}{}\x1b[0m",
                               if prompt
                               {
                                   if color
                                   {
                                       "\x1b[94m> \x1b[96m"
                                   }
                                   else
                                   {
                                       "> "
                                   }
                               }
                               else if color
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
                        if print_options.4
                        {
                            frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), print_options, prompt, color, prec);
                        }
                        else
                        {
                            print!("\x1B[2K\x1B[1G{}{}\x1b[0m",
                                   if prompt
                                   {
                                       if color
                                       {
                                           "\x1b[94m> \x1b[96m"
                                       }
                                       else
                                       {
                                           "> "
                                       }
                                   }
                                   else if color
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
                    color = !color;
                    continue;
                }
                "prompt" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    prompt = !prompt;
                    continue;
                }
                "deg" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.1 = true;
                    continue;
                }
                "rad" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.1 = false;
                    continue;
                }
                "rt" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.4 = !print_options.4;
                    continue;
                }
                "tau" =>
                {
                    print_options.3 = true;
                    write(&input, &mut file, &unmod_lines);
                    continue;
                }
                "pi" =>
                {
                    print_options.3 = false;
                    write(&input, &mut file, &unmod_lines);
                    continue;
                }
                "sci" | "scientific" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.0 = !print_options.0;
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
                    graph_options.4 = !graph_options.4;
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
                    graph_options.3 = r.chars().next().unwrap();
                    continue;
                }
                "base" =>
                {
                    print_options.2 = r.parse::<usize>().unwrap();
                    if print_options.2 > 36 || print_options.2 < 2
                    {
                        print_options.2 = 10;
                    }
                    continue;
                }
                "decimal" | "deci" | "decimals" =>
                {
                    print_options.5 = r.parse::<usize>().unwrap();
                    continue;
                }
                "prec" | "precision" =>
                {
                    // fix redefined vars
                    prec = if r == "0" { prec } else { r.parse::<u32>().unwrap() };
                    v = get_vars(allow_vars, prec);
                    vars = [v.clone(), vars[v.len()..].to_vec()].concat();
                    continue;
                }
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
            handles.push(graph(inputs, funcs, graph_options, print_options.1, prec, watch));
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
--print_options.3 fractions are shown in print_options.3 instead of pi\n\
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
- g: gravity\n\
- c: speed of light\n\
- h: planck's constant\n\
- e: euler's number\n\
- pi: pi\n\
- tau: tau (2pi)\n\
- phi: golden ratio\n\
- G: gravitational constant\n\
- ec: elementary charge\n\
- mp: proton mass\n\
- mn: neutron mass\n\
- me: electron mass\n\
- ev: electron volt\n\
- kc: coulomb's constant\n\
- na: avogadro's number\n\
- r: gas constant"
    );
}