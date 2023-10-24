mod complex;
mod fraction;
mod graph;
mod help;
mod math;
mod misc;
mod options;
mod parse;
mod print;
#[cfg(test)]
mod tests;
mod vars;
use crate::{
    graph::{can_graph, graph},
    misc::{clear, convert, get_terminal_width, read_single_char, write},
    options::{
        arg_opts, commands, equal_to, file_opts, set_commands, AngleType, AngleType::Radians,
    },
    parse::get_func,
    print::{print_answer, print_concurrent},
    vars::{get_vars, input_var},
};
use std::{
    env::{args, var},
    fs::{File, OpenOptions},
    io::{stdin, stdout, BufRead, BufReader, IsTerminal, Write},
    thread::JoinHandle,
    time::Instant,
};
#[derive(Clone, Copy)]
pub struct Options
{
    sci: bool,
    deg: AngleType,
    base: usize,
    tau: bool,
    polar: bool,
    frac: bool,
    real_time_output: bool,
    decimal_places: usize,
    color: bool,
    prompt: bool,
    comma: bool,
    prec: u32,
    frac_iter: usize,
    xr: (f64, f64),
    yr: (f64, f64),
    zr: (f64, f64),
    samples_2d: usize,
    samples_3d: (usize, usize),
    point_style: char,
    lines: bool,
    multi: bool,
    tabbed: bool,
    allow_vars: bool,
    small_e: bool,
    debug: bool,
    depth: bool,
}
impl Default for Options
{
    fn default() -> Self
    {
        Options {
            sci: false,
            deg: Radians,
            base: 10,
            tau: false,
            polar: false,
            frac: true,
            real_time_output: true,
            decimal_places: 12,
            color: true,
            prompt: true,
            comma: false,
            prec: 512,
            frac_iter: 50,
            xr: (-10.0, 10.0),
            yr: (-10.0, 10.0),
            zr: (-10.0, 10.0),
            samples_2d: 20000,
            samples_3d: (300, 300),
            point_style: '.',
            lines: false,
            multi: false,
            tabbed: false,
            allow_vars: true,
            small_e: false,
            debug: false,
            depth: false,
        }
    }
}
fn main()
{
    let mut options = Options::default();
    let mut watch = None;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    #[cfg(unix)]
    let file_path = &(var("HOME").unwrap() + "/.config/kalc.config");
    #[cfg(not(unix))]
    let file_path = &format!(
        "C:\\Users\\{}\\AppData\\Roaming\\kalc.config",
        var("USERNAME").unwrap()
    );
    let mut args = args().collect::<Vec<String>>();
    if file_opts(&mut options, file_path).is_err() || arg_opts(&mut options, &mut args).is_err()
    {
        std::process::exit(1);
    }
    let mut vars: Vec<[String; 2]> = if options.allow_vars
    {
        get_vars(options)
    }
    else
    {
        Vec::new()
    };
    let mut old = vars.clone();
    #[cfg(unix)]
    let file_path = &(var("HOME").unwrap() + "/.config/kalc.vars");
    #[cfg(not(unix))]
    let file_path = &format!(
        "C:\\Users\\{}\\AppData\\Roaming\\kalc.vars",
        var("USERNAME").unwrap()
    );
    if File::open(file_path).is_ok() && options.allow_vars
    {
        let lines = BufReader::new(File::open(file_path).unwrap())
            .lines()
            .map(|l| l.unwrap())
            .collect::<Vec<String>>();
        let mut split;
        for i in lines
        {
            split = i.splitn(2, '=');
            if split.clone().count() == 2
            {
                let l = split.next().unwrap().to_string();
                let r = split.next().unwrap().to_string();
                for (i, j) in vars.clone().iter().enumerate()
                {
                    if j[0].chars().count() <= l.chars().count()
                    {
                        vars.insert(i, [l, r]);
                        break;
                    }
                }
            }
        }
    }
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
    if !args.is_empty()
    {
        options.color = !options.color;
    }
    #[cfg(unix)]
    let file_path = &(var("HOME").unwrap() + "/.config/kalc.history");
    #[cfg(not(unix))]
    let file_path = &format!(
        "C:\\Users\\{}\\AppData\\Roaming\\kalc.history",
        var("USERNAME").unwrap()
    );
    if File::open(file_path).is_err()
    {
        File::create(file_path).unwrap();
    }
    let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
    let mut exit = false;
    let mut cut: Vec<char> = Vec::new();
    let mut stdout = stdout();
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
        let mut input = Vec::new();
        let mut frac = 0;
        let mut graphable = false;
        if !args.is_empty()
        {
            if options.debug
            {
                watch = Some(Instant::now());
            }
            input = args.first().unwrap().chars().collect();
            args.remove(0);
            print_answer(
                &input.iter().collect::<String>(),
                match get_func(
                    &input_var(
                        &input.iter().map(convert).collect::<String>(),
                        &vars,
                        &mut Vec::new(),
                        options,
                    ),
                    options,
                )
                {
                    Ok(f) => f,
                    Err(s) =>
                    {
                        println!("{}", s);
                        return;
                    }
                },
                options,
            );
            if let Some(time) = watch
            {
                print!(" {}", time.elapsed().as_nanos());
            }
            if !(can_graph(&input_var(
                &input.iter().collect::<String>(),
                &vars,
                &mut Vec::new(),
                options,
            )))
            {
                println!();
            }
            if args.is_empty()
            {
                exit = true;
            }
            graphable = true;
        }
        else
        {
            print!("\x1b[G\x1b[K");
            if options.prompt
            {
                if options.color
                {
                    print!("\x1b[94m> \x1b[0m");
                }
                else
                {
                    print!("> ");
                }
            }
            stdout.flush().unwrap();
            let mut current = Vec::new();
            let mut lines: Vec<String> = BufReader::new(File::open(file_path).unwrap())
                .lines()
                .map(|l| l.unwrap())
                .collect();
            let unmod_lines = lines.clone();
            let mut i = lines.len();
            let max = i;
            let mut placement = 0;
            let last = if i == 0
            {
                Vec::new()
            }
            else
            {
                lines[i - 1].chars().collect::<Vec<char>>()
            };
            let mut start = 0;
            let mut end = 0;
            loop
            {
                let c = read_single_char();
                if options.debug
                {
                    watch = Some(Instant::now());
                }
                match c
                {
                    '\n' =>
                    {
                        end = start + get_terminal_width() - if options.prompt { 3 } else { 1 };
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        if !options.real_time_output && !input.is_empty()
                        {
                            frac = print_concurrent(&input, &last, &vars, options, start, end);
                        }
                        if !(can_graph(&input_var(
                            &input.iter().collect::<String>(),
                            &vars,
                            &mut Vec::new(),
                            options,
                        )))
                        {
                            println!();
                        }
                        if !input.is_empty()
                        {
                            println!("{}", "\n".repeat(frac));
                        }
                        break;
                    }
                    '\x08' =>
                    {
                        //backspace
                        if placement - start == 0 && start != 0
                        {
                            start -= 1;
                        }
                        if placement != 0
                        {
                            placement -= 1;
                            input.remove(placement);
                        }
                        end = start + get_terminal_width() - if options.prompt { 3 } else { 1 };
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
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        frac = if options.real_time_output
                        {
                            print_concurrent(&input, &last, &vars, options, start, end)
                        }
                        else
                        {
                            clear(&input, start, end, options);
                            0
                        };
                        if let Some(time) = watch
                        {
                            let time = time.elapsed().as_nanos();
                            print!(
                                " {}{}",
                                time,
                                "\x08".repeat(
                                    time.to_string().len() + 1 + end - start - (placement - start)
                                )
                            );
                        }
                        else
                        {
                            print!("{}", "\x08".repeat(end - start - (placement - start)));
                        }
                    }
                    '\x7F' =>
                    {
                        //delete
                        if placement - start == 0 && start != 0
                        {
                            start -= 1;
                        }
                        if !input.is_empty()
                        {
                            input.remove(placement);
                        }
                        end = start + get_terminal_width() - if options.prompt { 3 } else { 1 };
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
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        frac = if options.real_time_output
                        {
                            print_concurrent(&input, &last, &vars, options, start, end)
                        }
                        else
                        {
                            clear(&input, start, end, options);
                            0
                        };
                        if let Some(time) = watch
                        {
                            let time = time.elapsed().as_nanos();
                            print!(
                                " {}{}",
                                time,
                                "\x08".repeat(
                                    time.to_string().len() + 1 + end - start - (placement - start)
                                )
                            );
                        }
                        else
                        {
                            print!("{}", "\x08".repeat(end - start - (placement - start)));
                        }
                    }
                    '\x11' =>
                    {
                        //end
                        placement = input.len();
                        end = input.len();
                        start = if get_terminal_width() - if options.prompt { 3 } else { 1 }
                            > input.len()
                        {
                            0
                        }
                        else
                        {
                            input.len()
                                - (get_terminal_width() - if options.prompt { 3 } else { 1 })
                        };
                        if options.real_time_output
                        {
                            frac = print_concurrent(&input, &last, &vars, options, start, end);
                        }
                        else
                        {
                            clear(&input, start, end, options);
                        }
                    }
                    '\x18' =>
                    {
                        //ctrl+u
                        cut = input.drain(..placement).collect();
                        end -= placement;
                        placement = 0;
                        if options.real_time_output
                        {
                            frac = print_concurrent(&input, &last, &vars, options, start, end);
                        }
                        else
                        {
                            clear(&input, start, end, options);
                        }
                        print!("{}", "\x08".repeat(end - start - (placement - start)));
                    }
                    '\x19' =>
                    {
                        //ctrl+k
                        cut = input.drain(placement..).collect();
                        end = input.len();
                        if options.real_time_output
                        {
                            frac = print_concurrent(&input, &last, &vars, options, start, end);
                        }
                        else
                        {
                            clear(&input, start, end, options);
                        }
                        print!("{}", "\x08".repeat(end - start - (placement - start)));
                    }
                    '\x17' =>
                    {
                        //ctrl+y
                        let mut cut = cut.clone();
                        end += cut.len();
                        cut.extend(input.drain(placement..));
                        input.extend(cut);
                        if options.real_time_output
                        {
                            frac = print_concurrent(&input, &last, &vars, options, start, end);
                        }
                        else
                        {
                            clear(&input, start, end, options);
                        }
                        print!("{}", "\x08".repeat(end - start - (placement - start)));
                    }
                    '\x16' =>
                    {
                        //ctrl+t
                        if placement + 1 < input.len()
                        {
                            let char = input.remove(placement);
                            input.insert(placement + 1, char);
                            if options.real_time_output
                            {
                                frac = print_concurrent(&input, &last, &vars, options, start, end);
                            }
                            else
                            {
                                clear(&input, start, end, options);
                            }
                            print!("{}", "\x08".repeat(end - start - (placement - start)));
                        }
                    }
                    '\x15' =>
                    {
                        //ctrl+l
                        print!("\x1b[H\x1b[J");
                        if options.real_time_output && !input.is_empty()
                        {
                            frac = print_concurrent(&input, &last, &vars, options, start, end);
                        }
                        else
                        {
                            clear(&input, start, end, options);
                        }
                        print!("{}", "\x08".repeat(end - start - (placement - start)));
                    }
                    '\x10' =>
                    {
                        //home
                        placement = 0;
                        start = 0;
                        end = if get_terminal_width() - if options.prompt { 3 } else { 1 }
                            > input.len()
                        {
                            input.len()
                        }
                        else
                        {
                            get_terminal_width() - if options.prompt { 3 } else { 1 }
                        };
                        if options.real_time_output && !input.is_empty()
                        {
                            frac = print_concurrent(&input, &last, &vars, options, start, end);
                        }
                        else
                        {
                            clear(&input, start, end, options);
                        }
                        print!("{}", "\x08".repeat(end - start - (placement - start)));
                    }
                    '\x1D' =>
                    {
                        //up history
                        i -= if i > 0 { 1 } else { 0 };
                        input = lines[i].clone().chars().collect::<Vec<char>>();
                        placement = input.len();
                        end = input.len();
                        start = if get_terminal_width() - if options.prompt { 3 } else { 1 }
                            > input.len()
                        {
                            0
                        }
                        else
                        {
                            input.len()
                                - (get_terminal_width() - if options.prompt { 3 } else { 1 })
                        };
                        if options.real_time_output
                        {
                            frac = print_concurrent(&input, &last, &vars, options, start, end);
                        }
                        else
                        {
                            clear(&input, start, end, options);
                        }
                    }
                    '\x1E' =>
                    {
                        //down history
                        i += 1;
                        if i >= max
                        {
                            input = current.clone();
                            i = max;
                        }
                        else
                        {
                            input = lines[i].clone().chars().collect::<Vec<char>>();
                        }
                        placement = input.len();
                        end = input.len();
                        start = if get_terminal_width() - if options.prompt { 3 } else { 1 }
                            > input.len()
                        {
                            0
                        }
                        else
                        {
                            input.len()
                                - (get_terminal_width() - if options.prompt { 3 } else { 1 })
                        };
                        if options.real_time_output
                        {
                            frac = print_concurrent(&input, &last, &vars, options, start, end);
                        }
                        else
                        {
                            clear(&input, start, end, options);
                        }
                    }
                    '\x1B' =>
                    {
                        //go left
                        if placement - start == 0 && placement != 0 && start != 0
                        {
                            start -= 1;
                            placement -= 1;
                            end = start + get_terminal_width() - if options.prompt { 3 } else { 1 };
                            if end > input.len()
                            {
                                end = input.len()
                            }
                            clear(&input, start, end, options);
                            print!("{}", "\x08".repeat(end - start - (placement - start)))
                        }
                        else if placement != 0
                        {
                            placement -= 1;
                            print!("\x08");
                        }
                    }
                    '\x1C' =>
                    {
                        //go right
                        end = start + get_terminal_width() - if options.prompt { 3 } else { 1 };
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        if placement == end && end != input.len()
                        {
                            start += 1;
                            placement += 1;
                            end += 1;
                            clear(&input, start, end, options);
                        }
                        else if placement != input.len()
                        {
                            placement += 1;
                            print!("\x1b[C")
                        }
                    }
                    '\x12' =>
                    {
                        //ctrl+left
                        if placement != 0
                        {
                            let s = placement;
                            let mut hit = false;
                            for (i, j) in input[..s].iter().enumerate().rev()
                            {
                                if !j.is_alphanumeric()
                                {
                                    if hit
                                    {
                                        placement = i + 1;
                                        break;
                                    }
                                }
                                else
                                {
                                    hit = true;
                                }
                            }
                            if placement <= start
                            {
                                end -= start - placement;
                                start = start - (start - placement);
                                clear(&input, start, end, options);
                                print!(
                                    "{}",
                                    "\x08".repeat(
                                        get_terminal_width() - if options.prompt { 3 } else { 1 }
                                    )
                                );
                            }
                            else if placement == s
                            {
                                placement = 0;
                                print!("{}", "\x08".repeat(s));
                            }
                            else
                            {
                                print!("{}", "\x08".repeat(s - placement));
                            }
                        }
                    }
                    '\x13' =>
                    {
                        //ctrl+right
                        if placement != input.len()
                        {
                            let s = placement;
                            let mut hit = false;
                            for (i, j) in input[s + 1..].iter().enumerate()
                            {
                                if !j.is_alphanumeric()
                                {
                                    if hit
                                    {
                                        placement += i + 1;
                                        break;
                                    }
                                }
                                else
                                {
                                    hit = true;
                                }
                            }
                            if placement >= end
                            {
                                start += placement - end;
                                end = end + (placement - end);
                                clear(&input, start, end, options);
                            }
                            else if placement == s
                            {
                                placement = input.len();
                                print!("{}", "\x1b[C".repeat(input.len() - s));
                            }
                            else
                            {
                                print!("{}", "\x1b[C".repeat(placement - s));
                            }
                        }
                    }
                    '\0' =>
                    {}
                    _ =>
                    {
                        input.insert(placement, c);
                        placement += 1;
                        end = start + get_terminal_width() - if options.prompt { 3 } else { 1 } + 1;
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        else if placement == end
                        {
                            start += 1;
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
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        if options.real_time_output
                        {
                            frac = print_concurrent(&input, &last, &vars, options, start, end);
                        }
                        else
                        {
                            clear(&input, start, end, options)
                        }
                        if let Some(time) = watch
                        {
                            let time = time.elapsed().as_nanos();
                            print!(
                                " {}{}",
                                time,
                                "\x08".repeat(
                                    time.to_string().len() + 1 + end - start - (placement - start)
                                )
                            );
                        }
                        else if placement != input.len()
                        {
                            print!("{}", "\x08".repeat(end - start - (placement - start)));
                        }
                    }
                }
                stdout.flush().unwrap();
            }
            if input.is_empty()
            {
                continue;
            }
            commands(
                &mut options,
                &mut watch,
                &mut vars,
                &mut old,
                &mut lines,
                &mut input,
                &mut stdout,
            );
            write(
                &input
                    .iter()
                    .collect::<String>()
                    .replace("small_e", "smalle")
                    .replace("frac_iter", "fraciter")
                    .replace('_', &format!("({})", last.iter().collect::<String>()))
                    .replace("smalle", "small_e")
                    .replace("fraciter", "frac_iter"),
                &mut file,
                &unmod_lines,
            );
            if input.ends_with(&['='])
            {
                equal_to(
                    &mut options,
                    &mut vars,
                    &input[..input.len() - 1].iter().collect::<String>(),
                    &last.iter().collect::<String>(),
                )
            }
            else if input
                .iter()
                .collect::<String>()
                .replace("==", "")
                .replace("!=", "")
                .replace(">=", "")
                .replace("<=", "")
                .contains('=')
            {
                print!("\x1b[J");
                stdout.flush().unwrap();
                let n = input.iter().collect::<String>();
                let mut split = n.splitn(2, '=');
                let s = split.next().unwrap().replace(' ', "");
                let l = s;
                let r = split.next().unwrap();
                if l.is_empty() || set_commands(&mut options, &mut vars, &mut old, &l, r)
                {
                    continue;
                }
                if l.contains('(')
                {
                    let mut s = l.split('(').next().iter().copied().collect::<String>();
                    s.push('(');
                    let recur_test = r.split(&s);
                    let count = recur_test.clone().count();
                    for (i, s) in recur_test.enumerate()
                    {
                        if i + 1 != count
                            && (s.is_empty() || !s.chars().last().unwrap().is_alphabetic())
                        {
                            println!("recursive functions not supported");
                            continue 'main;
                        }
                    }
                }
                for (i, v) in vars.iter().enumerate()
                {
                    if v[0].split('(').next() == l.split('(').next()
                        && v[0].find(',').iter().count() == l.find(',').iter().count()
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
                for (i, j) in vars.iter().enumerate()
                {
                    if j[0].chars().count() <= l.chars().count()
                    {
                        vars.insert(i, [l.to_string(), r.to_string()]);
                        break;
                    }
                }
                if vars.is_empty()
                {
                    vars.push([l.to_string(), r.to_string()]);
                }
            }
            else
            {
                graphable = true;
            }
        }
        if graphable
            && input.iter().collect::<String>() != "history"
            && (input.contains(&'#')
                || can_graph(&input_var(
                    &input.iter().collect::<String>(),
                    &vars,
                    &mut Vec::new(),
                    options,
                )))
        {
            print!("\x1b[G\x1b[K");
            stdout.flush().unwrap();
            let mut inputs: Vec<String> = input
                .iter()
                .collect::<String>()
                .split('#')
                .map(String::from)
                .collect();
            let unmod = inputs.clone();
            let mut funcs = Vec::new();
            for i in inputs.iter_mut()
            {
                if i.is_empty()
                {
                    continue;
                }
                *i = input_var(i, &vars, &mut Vec::new(), options)
                    .replace("zeta", "##ta##")
                    .replace("normalize", "##ma##")
                    .replace('z', "(x+y*i)")
                    .replace("##ta##", "zeta")
                    .replace("##ma##", "normalize");
                funcs.push(match get_func(i, options)
                {
                    Ok(f) => f,
                    _ => continue 'main,
                });
            }
            handles.push(graph(inputs, unmod, funcs, options, watch));
        }
    }
}