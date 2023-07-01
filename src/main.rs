mod fraction;
mod graph;
mod math;
mod options;
mod parse;
mod print;
#[cfg(test)]
mod tests;
use parse::{get_func, get_vars, input_var};
use std::{
    env::{args, var},
    fs::{File, OpenOptions},
    io::{stdin, stdout, BufRead, BufReader, IsTerminal, Write},
    thread::JoinHandle,
};
use graph::graph;
use print::{get_output, print_answer, print_concurrent};
use options::{arg_opts, file_opts};
use math::{
    do_math,
    NumStr::{Num, Str, Vector},
};
#[cfg(not(unix))]
use {
    console::{Key, Term},
    term_size::dimensions,
};
#[cfg(unix)]
use {
    libc::{ioctl, tcgetattr, tcsetattr, winsize, ECHO, ICANON, STDOUT_FILENO, TCSANOW, TIOCGWINSZ, VMIN, VTIME},
    std::io::Read,
    std::os::fd::AsRawFd,
};
// allow f16/f32/f64/f128 instead of arbitary precision for performance reasons
// fix 0's and infinities of sin, cos, tan and cis
// complex vectors
// gui support
// support unit conversions
// allow units to be used in the input, and be outputted
fn main()
{
    let mut graph_options = GraphOptions::default();
    let mut print_options = PrintOptions::default();
    let mut watch = None;
    let mut allow_vars = true;
    let mut debug = false;
    let mut handles:Vec<JoinHandle<()>> = Vec::new();
    #[cfg(unix)]
    let file_path = &(var("HOME").unwrap() + "/.config/kalc.config");
    #[cfg(not(unix))]
    let file_path = &format!("C:\\Users\\{}\\AppData\\Roaming\\kalc.config", var("USERNAME").unwrap());
    let mut args = args().collect::<Vec<String>>();
    if file_opts(&mut graph_options, &mut print_options, &mut allow_vars, &mut debug, file_path) || arg_opts(&mut graph_options, &mut print_options, &mut allow_vars, &mut debug, &mut args)
    {
        std::process::exit(1);
    }
    let mut vars:Vec<[String; 2]> = if allow_vars { get_vars(print_options.prec) } else { Vec::new() };
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
    let (mut c, mut i, mut max, mut frac, mut l, mut r, mut split, mut funcs, mut v, mut start, mut end, mut placement);
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
        frac = 0;
        if !args.is_empty()
        {
            if debug
            {
                watch = Some(std::time::Instant::now());
            }
            input = args.first().unwrap().replace('_', &format!("({})", last));
            args.remove(0);
            print_answer(&input,
                         match get_func(&input_var(&input.replace('π', "pi").replace('τ', "tau"), &vars, None), print_options.prec, print_options.deg)
                         {
                             Ok(f) => f,
                             Err(()) =>
                             {
                                 println!("Invalid function.");
                                 return;
                             }
                         },
                         print_options);
            if let Some(time) = watch
            {
                print!(" {}", time.elapsed().as_nanos());
            }
            if !(input.is_empty()
                 || input.contains('#')
                 || (input.replace("exp", "").replace("}x{", "").replace("]x[", "").contains('x') && vars.iter().all(|i| i[0] != "x"))
                 || (input.contains('y') && vars.iter().all(|i| i[0] != "y"))
                 || (input.replace("zeta", "").contains('z') && vars.iter().all(|i| i[0] != "z"))
                 || input.replace("==", "").replace("!=", "").replace(">=", "").replace("<=", "").contains('='))
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
            placement = 0;
            last = lines.last().unwrap_or(&String::new()).clone();
            start = 0;
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
                        end = start + get_terminal_width() - if print_options.prompt { 3 } else { 0 };
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        if !print_options.real_time_output
                        {
                            frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars, None), print_options, start, end);
                        }
                        if !(input.is_empty()
                             || input.contains('#')
                             || (input.replace("exp", "").replace("}x{", "").replace("]x[", "").contains('x') && vars.iter().all(|i| i[0] != "x"))
                             || (input.contains('y') && vars.iter().all(|i| i[0] != "y"))
                             || (input.replace("zeta", "").contains('z') && vars.iter().all(|i| i[0] != "z"))
                             || input.replace("==", "").replace("!=", "").replace(">=", "").replace("<=", "").contains('='))
                        {
                            println!();
                        }
                        println!("{}", "\n".repeat(frac));
                        break;
                    }
                    '\x08' =>
                    {
                        if placement - start == 0 && start != 0
                        {
                            start -= 1;
                        }
                        if placement == 0
                        {
                            if input.is_empty()
                            {
                                print!("\x1B[0J\x1B[2K\x1B[1G{}",
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
                        placement -= 1;
                        input.remove(placement);
                        end = start + get_terminal_width() - if print_options.prompt { 3 } else { 0 };
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        if i == max
                        {
                            current = input.clone();
                        }
                        else
                        {
                            lines[i as usize] = input.clone();
                        }
                        frac = if input.is_empty()
                        {
                            0
                        }
                        else if print_options.real_time_output
                        {
                            print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars, None), print_options, start, end)
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
                                   &input[start..end]);
                            0
                        };
                        if let Some(time) = watch
                        {
                            let time = time.elapsed().as_nanos();
                            print!(" {}{}", time, "\x08".repeat(time.to_string().len() + 1 + end - start - (placement - start)));
                        }
                        else
                        {
                            print!("{}", "\x08".repeat(end - start - (placement - start)));
                        }
                        if placement == 0 && input.is_empty()
                        {
                            print!("\x1B[0J\x1B[2K\x1B[1G{}",
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
                        placement = input.len();
                        end = input.len();
                        start = if get_terminal_width() - if print_options.prompt { 3 } else { 0 } > input.len()
                        {
                            0
                        }
                        else
                        {
                            input.len() - (get_terminal_width() - if print_options.prompt { 3 } else { 0 })
                        };
                        if print_options.real_time_output
                        {
                            frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars, None), print_options, start, end);
                        }
                        else
                        {
                            print!("\x1B[0J\x1B[2K\x1B[1G{}{}\x1b[0m",
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
                                   &input[start..]);
                        }
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
                                print!("\x1B[0J\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A\x1B[2K\x1B[1G{}",
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
                                placement = 0;
                                start = 0;
                                continue 'outer;
                            }
                        }
                        else
                        {
                            input = lines[i as usize].clone();
                        }
                        placement = input.len();
                        end = input.len();
                        start = if get_terminal_width() - if print_options.prompt { 3 } else { 0 } > input.len()
                        {
                            0
                        }
                        else
                        {
                            input.len() - (get_terminal_width() - if print_options.prompt { 3 } else { 0 })
                        };
                        if print_options.real_time_output
                        {
                            frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars, None), print_options, start, end);
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
                                   &input[start..]);
                        }
                    }
                    '\x1B' =>
                    {
                        // go left
                        if placement - start == 0 && placement != 0 && start != 0
                        {
                            start -= 1;
                            placement -= 1;
                            end = start + get_terminal_width() - if print_options.prompt { 3 } else { 0 };
                            if end > input.len()
                            {
                                end = input.len()
                            }
                            print!("\x1B[2K\x1B[1G{}{}\x1b[0m{}",
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
                                   &input[start..end],
                                   "\x08".repeat(end - start - (placement - start)));
                            stdout().flush().unwrap();
                        }
                        else if placement != 0
                        {
                            placement -= 1;
                            print!("\x08");
                        }
                    }
                    '\x1C' =>
                    {
                        // go right
                        end = start + get_terminal_width() - if print_options.prompt { 3 } else { 0 };
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        if placement == end && end != input.len()
                        {
                            start += 1;
                            placement += 1;
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
                                   &input[start..end + 1]);
                        }
                        else if placement != input.len()
                        {
                            placement += 1;
                            print!("\x1b[1C")
                        }
                    }
                    '\0' =>
                    {}
                    _ =>
                    {
                        if c == 'π'
                        {
                            input.insert_str(placement, "pi");
                            placement += 2;
                        }
                        else if c == 'τ'
                        {
                            input.insert_str(placement, "tau");
                            placement += 3;
                        }
                        else
                        {
                            input.insert(placement, c);
                            placement += 1;
                        }
                        end = start + get_terminal_width() - if print_options.prompt { 3 } else { 0 } + 1;
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        else if placement == end
                        {
                            if c == 'π'
                            {
                                start += 2;
                            }
                            else if c == 'τ'
                            {
                                start += 3;
                            }
                            else
                            {
                                start += 1;
                            }
                        }
                        else if c == 'π'
                        {
                            end -= 2;
                        }
                        else if c == 'τ'
                        {
                            end -= 3;
                        }
                        else
                        {
                            end -= 1;
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
                            frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars, None), print_options, start, end);
                        }
                        else
                        {
                            print!("\x1B[0J\x1B[2K\x1B[1G{}{}\x1b[0m",
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
                                   &input[start..end]);
                        }
                        if let Some(time) = watch
                        {
                            let time = time.elapsed().as_nanos();
                            print!(" {}{}", time, "\x08".repeat(time.to_string().len() + 1 + end - start - (placement - start)));
                        }
                        else if placement != input.len()
                        {
                            print!("{}", "\x08".repeat(end - start - (placement - start)));
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
                    print_options.deg = 1;
                    continue;
                }
                "rad" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.deg = 0;
                    continue;
                }
                "grad" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.deg = 2;
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
                "polar" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.polar = !print_options.polar;
                    continue;
                }
                "frac" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.frac = !print_options.frac;
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
                    let mut n;
                    for v in vars.iter()
                    {
                        if v[0].contains('(')
                        {
                            println!("{}={}", v[0], v[1]);
                        }
                        else
                        {
                            n = get_output(&print_options,
                                           &do_math(get_func(&input_var(&v[1], &vars, Some(&v[0])), print_options.prec, print_options.deg).unwrap(),
                                                    print_options.deg,
                                                    print_options.prec).unwrap()
                                                                       .num()
                                                                       .unwrap());
                            println!("{}={}{}", v[0], n.0, n.1);
                        }
                    }
                    continue;
                }
                "lvars" =>
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
        if input.ends_with('=')
        {
            l = &input[..input.len() - 1];
            match l
            {
                "point" => println!("{}", graph_options.point_style),
                "base" => println!("{}", print_options.base),
                "decimal" | "deci" | "decimals" => println!("{}", print_options.decimal_places),
                "prec" | "precision" => println!("{}", print_options.prec),
                "xr" => println!("{},{}", graph_options.xr[0], graph_options.xr[1]),
                "yr" => println!("{},{}", graph_options.yr[0], graph_options.yr[1]),
                "zr" => println!("{},{}", graph_options.zr[0], graph_options.zr[1]),
                "frac_iter" => println!("{}", print_options.frac_iter),
                "2d" => println!("{}", graph_options.samples_2d),
                "3d" => println!("{}", graph_options.samples_3d),
                _ =>
                {
                    for i in get_func(&input_var(l, &vars, None), print_options.prec, print_options.deg).unwrap()
                    {
                        match i
                        {
                            Num(n) =>
                            {
                                let n = get_output(&print_options, &n);
                                print!("{}{}\x1b[0m", n.0, n.1)
                            }
                            Vector(n) =>
                            {
                                let mut str = String::new();
                                let mut num;
                                for i in n
                                {
                                    num = get_output(&print_options, &i);
                                    str.push_str(&format!("{}{}\x1b[0m,", num.0, num.1));
                                }
                                str.pop();
                                print!("{{{}}}", str)
                            }
                            Str(n) => print!("{}", n),
                        }
                    }
                    println!();
                }
            }
            continue;
        }
        if input.replace("==", "").replace("!=", "").replace(">=", "").replace("<=", "").contains('=')
        {
            print!("\x1B[0J");
            stdout().flush().unwrap();
            split = input.splitn(2, '=');
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
                    if r == "-1"
                    {
                        print_options.decimal_places = usize::MAX - 1;
                    }
                    else if r == "-2"
                    {
                        print_options.decimal_places = usize::MAX;
                    }
                    else
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
                    }
                    continue;
                }
                "prec" | "precision" =>
                {
                    print_options.prec = if r == "0"
                    {
                        println!("Invalid precision");
                        print_options.prec
                    }
                    else
                    {
                        match r.parse::<u32>()
                        {
                            Ok(n) => n,
                            Err(_) =>
                            {
                                println!("Invalid precision");
                                print_options.prec
                            }
                        }
                    };
                    if allow_vars
                    {
                        v = get_vars(print_options.prec);
                        vars = [v.clone(), vars[v.len()..].to_vec()].concat();
                    }
                    continue;
                }
                "xr" =>
                {
                    if r.contains(',')
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
                }
                "yr" =>
                {
                    if r.contains(',')
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
                }
                "zr" =>
                {
                    if r.contains(',')
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
                }
                "frac_iter" =>
                {
                    print_options.frac_iter = match r.parse::<usize>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid frac_iter");
                            print_options.frac_iter
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
        else if input.contains('#')
                  || (input.replace("exp", "").replace("}x{", "").replace("]x[", "").contains('x') && vars.iter().all(|i| i[0] != "x"))
                  || (input.replace("zeta", "").contains('z') && vars.iter().all(|i| i[0] != "z"))
        {
            input = input.replace("zeta", "##ta##").replace('z', "(x+y*i)").replace("##ta##", "zeta");
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            inputs = input.split('#').map(String::from).collect();
            funcs = Vec::new();
            for i in &inputs
            {
                if i.is_empty()
                {
                    continue;
                }
                funcs.push(match get_func(&input_var(i, &vars, None), print_options.prec, print_options.deg)
                     {
                         Ok(f) => f,
                         _ => continue 'main,
                     });
            }
            handles.push(graph(inputs, funcs, graph_options, print_options.deg, print_options.prec, watch));
            continue;
        }
    }
}
#[cfg(unix)]
fn get_terminal_width() -> usize
{
    unsafe {
        let mut size:winsize = std::mem::zeroed();
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) == 0 && size.ws_col != 0
        {
            size.ws_col as usize
        }
        else
        {
            80
        }
    }
}
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
                _ => '\0',
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
                _ => '\0',
            }
        }
        127 => '\x08',
        b'a'..=b'z'
        | b'A'..=b'Z'
        | b'0'..=b'9'
        | b'+'
        | b'-'
        | b'*'
        | b'/'
        | b'^'
        | b'('
        | b')'
        | b'.'
        | b'='
        | b','
        | b'#'
        | b'|'
        | b'&'
        | b'!'
        | b'%'
        | b'_'
        | b'<'
        | b'>'
        | b' '
        | b'\n'
        | b'['
        | b']'
        | b'{'
        | b'}' => input[0] as char,
        _ => '\0',
    }
}
#[cfg(not(unix))]
fn get_terminal_width() -> usize
{
    if let Some((width, _)) = dimensions()
    {
        width
    }
    else
    {
        80
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
               || c == '['
               || c == ']'
               || c == '{'
               || c == '}'
            {
                c
            }
            else
            {
                '\0'
            }
        }
        Key::Enter => '\n',
        Key::Backspace => '\x08',
        Key::ArrowLeft => '\x1B',
        Key::ArrowRight => '\x1C',
        Key::ArrowUp => '\x1D',
        Key::ArrowDown => '\x1E',
        _ => '\0',
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
--tau fractions are shown in tau instead of pi\n\
--deg compute in degrees\n\
--rad compute in radians\n\
--grad compute in gradians\n\
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
--polar toggles displaying polar vectors\n\
--frac toggles fraction display\n\
--frac_iter [num] how many iterations to check for fractions\n\
--prec [num] sets the precision\n\
--deci [num] sets how many decimals to display, -1 for length of terminal, -2 for maximum decimal places, may need to up precision for more decimals\n\
--def ignores config file\n\
--debug displays computation time in nanoseconds\n\n\
- Type \"exit\" to exit the program\n\
- Type \"clear\" to clear the screen\n\
- Type \"help\" to see this message\n\
- Type \"history\" to see the history of calculations\n\
- Type \"deg\" to switch to degrees mode\n\
- Type \"rad\" to switch to radians mode\n\
- Type \"grad\" to switch to gradians mode\n\
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
- Type \"deci=[num]\" to set how many decimals to display, -1 for length of terminal, -2 for maximum decimal places, may need to up precision for more decimals\n\
- Type \"point=[char]\" to set the point style for graphing\n\
- Type \"sci\" to toggle scientific notation\n\
- Type \"vars\" to list all variables\n\
- Type \"lvars\" to list all variables without equating them\n\
- Type \"base=[num]\" to set the number base (2-36)\n\
- Type \"_\" to use the previous answer\n\
- Type \"a=[num]\" to define a variable\n\
- Type \"f(x)=...\" to define a function\n\
- Type \"f(x,y)=...\" to define a 2 variable function\n\
- Type \"f(x,y,z...)=...\" to define a multi variable function\n\
- Type \"...=\" display parsed input, show values of stuff like xr/deci/prec etc\n\
- Type \"f...=null\" to delete a function or variable\n\
- Type \"{{x,y,z...}}\" to define a cartesian vector\n\
- Type \"[radius,theta,phi]\" to define a polar vector (same as car{{vec}})\n\
- Type \"{{vec}}#\" to graph a vector\n\
- Type \"number#\" to graph a complex number\n\
- Type \"polar\" to toggle polar output\n\
- Type \"frac\" to toggle fraction display\n\
- Type \"frac_iter=[num]\" how many iterations to check for fractions\n\
- Type \"debug\" toggles displaying computation time in nanoseconds\n\n\
Operators:\n\
- +, -, *, /, ^, %, <, >, <=, >=\n\
- !x (subfact), x! (fact)\n\
- && (and), || (or), == (equals), != (not equals)\n\
- >> (right shift), << (left shift)\n\n\
Trigonometric functions:\n\
- sin, cos, tan, asin, acos, atan, atan(x,y)\n\
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
- deg(to_degrees), rad(to_radians), grad(to_gradians)\n\
- re(real part), im(imaginary part)\n\n\
Vector operations/functions:\n\
- dot product: {{vec1}}.{{vec2}}\n\
- cross product: {{vec1}}x{{vec2}}\n\
- magnitude: |{{vec}}|\n\
- normal operations: {{vec}}^{{vec}}, {{vec}}*{{vec}}, {{vec}}/{{vec}}, {{vec}}+{{vec}}, {{vec}}-{{vec}} (works with scalars too)\n\
- convert to polar: pol{{vec}} outputs (radius, theta, phi)\n\
- convert to cartesian: car{{vec}} outputs (x, y, z)\n\n\
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
    deg:u8, // 0=rad,1=deg,2=grad
    base:usize,
    tau:bool,
    polar:bool,
    frac:bool,
    real_time_output:bool,
    decimal_places:usize,
    color:bool,
    prompt:bool,
    comma:bool,
    prec:u32,
    frac_iter:usize,
}
impl Default for PrintOptions
{
    fn default() -> Self
    {
        PrintOptions { sci:false,
                       deg:0,
                       base:10,
                       tau:false,
                       polar:false,
                       frac:true,
                       real_time_output:true,
                       decimal_places:12,
                       color:true,
                       prompt:true,
                       comma:false,
                       prec:512,
                       frac_iter:50 }
    }
}
impl Default for GraphOptions
{
    fn default() -> Self
    {
        GraphOptions { xr:[-10.0, 10.0],
                       yr:[-10.0, 10.0],
                       zr:[-10.0, 10.0],
                       samples_2d:20000.0,
                       samples_3d:400.0,
                       point_style:'.',
                       lines:false }
    }
}