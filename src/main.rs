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
    complex::NumStr::{Matrix, Num, Str, Vector},
    graph::{can_graph, graph},
    help::{get_help, help},
    math::do_math,
    misc::{clear, convert, get_terminal_width, read_single_char, write},
    options::{
        arg_opts, file_opts, AngleType,
        AngleType::{Degrees, Gradians, Radians},
    },
    parse::get_func,
    print::{get_output, print_answer, print_concurrent},
    vars::{get_vars, input_var},
};
use std::{
    env::{args, var},
    fs::{File, OpenOptions},
    io::{stdin, stdout, BufRead, BufReader, IsTerminal, Write},
    thread::JoinHandle,
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
    xr: [f64; 2],
    yr: [f64; 2],
    zr: [f64; 2],
    samples_2d: f64,
    samples_3d: f64,
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
            xr: [-10.0, 10.0],
            yr: [-10.0, 10.0],
            zr: [-10.0, 10.0],
            samples_2d: 20000.0,
            samples_3d: 400.0,
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
    if file_opts(&mut options, file_path) || arg_opts(&mut options, &mut args)
    {
        std::process::exit(1);
    }
    let mut vars: Vec<[String; 2]> = if options.allow_vars
    {
        get_vars(options.prec)
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
    let mut lines: Vec<String>;
    let mut unmod_lines: Vec<String>;
    let mut last: Vec<char> = Vec::new();
    let mut input: Vec<char> = Vec::new();
    let mut current: Vec<char> = Vec::new();
    let mut inputs: Vec<String>;
    let (
        mut c,
        mut i,
        mut max,
        mut frac,
        mut l,
        mut r,
        mut split,
        mut funcs,
        mut v,
        mut start,
        mut placement,
    );
    let mut end = 0;
    let mut exit = false;
    let mut cut: Vec<char> = Vec::new();
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
            if options.debug
            {
                watch = Some(std::time::Instant::now());
            }
            input = args.first().unwrap().chars().collect();
            args.remove(0);
            print_answer(
                &input.iter().collect::<String>(),
                match get_func(
                    &input_var(
                        &input.iter().map(convert).collect::<String>(),
                        &vars,
                        None,
                        options,
                    )
                    .replace('_', &format!("({})", last.iter().collect::<String>())),
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
                None,
                options,
            )))
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
            if options.prompt
            {
                print!(
                    "\x1b[G\x1b[K{}> \x1b[0m",
                    if options.color { "\x1b[94m" } else { "" }
                );
            }
            else
            {
                print!("\x1b[G\x1b[K");
            }
            stdout().flush().unwrap();
            current.clear();
            lines = BufReader::new(File::open(file_path).unwrap())
                .lines()
                .map(|l| l.unwrap())
                .collect();
            unmod_lines = lines.clone();
            i = lines.len() as i32;
            max = i;
            placement = 0;
            last = lines
                .last()
                .unwrap_or(&String::new())
                .clone()
                .chars()
                .collect::<Vec<char>>();
            start = 0;
            loop
            {
                c = read_single_char();
                if options.debug
                {
                    watch = Some(std::time::Instant::now());
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
                            None,
                            options,
                        )))
                        {
                            println!();
                        }
                        if !input.is_empty()
                        {
                            println!("{}", "\n".repeat(frac));
                        }
                        end = 0;
                        break;
                    }
                    '\x08' =>
                    {
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
                            lines[i as usize] = input.clone().iter().collect::<String>();
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
                    //Delete
                    {
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
                            lines[i as usize] = input.clone().iter().collect::<String>();
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
                        // up history
                        i -= if i > 0 { 1 } else { 0 };
                        input = lines[i as usize].clone().chars().collect::<Vec<char>>();
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
                        // down history
                        i += 1;
                        if i >= max
                        {
                            input = current.clone();
                            i = max;
                        }
                        else
                        {
                            input = lines[i as usize].clone().chars().collect::<Vec<char>>();
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
                        // go left
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
                        // go right
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
                            lines[i as usize] = input.clone().iter().collect::<String>();
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
                stdout().flush().unwrap();
            }
            if input.is_empty()
            {
                continue;
            }
            match input.iter().collect::<String>().as_str()
            {
                "color" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.color = !options.color;
                }
                "prompt" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.prompt = !options.prompt;
                }
                "depth" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.depth = !options.depth;
                }
                "deg" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.deg = Degrees;
                }
                "rad" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.deg = Radians;
                }
                "grad" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.deg = Gradians;
                }
                "rt" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.real_time_output = !options.real_time_output;
                }
                "tau" => options.tau = true,
                "pi" => options.tau = false,
                "small_e" => options.small_e = !options.small_e,
                "sci" | "scientific" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.sci = !options.sci;
                }
                "clear" =>
                {
                    print!("\x1b[H\x1b[J");
                    stdout().flush().unwrap();
                }
                "debug" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.debug = !options.debug;
                    watch = None;
                }
                "help" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    help();
                    continue;
                }
                "line" | "lines" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.lines = !options.lines;
                }
                "polar" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.polar = !options.polar;
                }
                "frac" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.frac = !options.frac;
                }
                "multi" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.multi = !options.multi;
                }
                "tabbed" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.tabbed = !options.tabbed;
                }
                "comma" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    options.comma = !options.comma;
                }
                "history" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    for l in lines
                    {
                        println!("{}", l);
                    }
                    continue;
                }
                "vars" | "variables" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    for v in vars.iter()
                    {
                        if v[0].contains('(')
                        {
                            println!("{}={}", v[0], v[1]);
                        }
                        else
                        {
                            match &do_math(
                                get_func(&input_var(&v[1], &vars, Some(&v[0]), options), options)
                                    .unwrap(),
                                options.deg,
                                options.prec,
                            )
                            .unwrap()
                            {
                                Num(n) =>
                                {
                                    let n = get_output(&options, n);
                                    println!("{}={}{}", v[0], n.0, n.1)
                                }
                                Vector(m) =>
                                {
                                    let mut st = String::new();
                                    for i in m
                                    {
                                        let n = get_output(&options, i);
                                        st.push_str(&n.0);
                                        st.push_str(&n.1);
                                        st.push(',');
                                    }
                                    println!("{}={{{}}}", v[0], st.trim_end_matches(','))
                                }
                                Matrix(m) =>
                                {
                                    let mut st = String::new();
                                    for i in m
                                    {
                                        st.push('{');
                                        for g in i
                                        {
                                            let n = get_output(&options, g);
                                            st.push_str(&n.0);
                                            st.push_str(&n.1);
                                            st.push(',');
                                        }
                                        st = st.trim_end_matches(',').to_string();
                                        st.push('}');
                                        st.push(',');
                                    }
                                    println!("{}={{{}}}", v[0], st.trim_end_matches(','))
                                }
                                _ => continue,
                            }
                        }
                    }
                }
                "lvars" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    for v in vars.iter()
                    {
                        println!("{}={}", v[0], v[1]);
                    }
                }
                "version" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                    continue;
                }
                "exit" | "quit" | "break" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    stdout().flush().unwrap();
                    break;
                }
                _ =>
                {
                    let n = input.iter().collect::<String>();
                    split = n.splitn(2, ' ');
                    let next = split.next().unwrap();
                    if next == "history"
                    {
                        print!("\x1b[A\x1b[G\x1b[K");
                        stdout().flush().unwrap();
                        r = split.next().unwrap();
                        for i in lines
                        {
                            if i.contains(r)
                            {
                                println!("{}", i);
                            }
                        }
                        continue;
                    }
                    if next == "help"
                    {
                        print!("\x1b[A\x1b[G\x1b[K");
                        stdout().flush().unwrap();
                        get_help(split.next().unwrap());
                        continue;
                    }
                }
            }
            write(
                &input
                    .iter()
                    .collect::<String>()
                    .replace('_', &format!("({})", last.iter().collect::<String>())),
                &mut file,
                &unmod_lines,
            );
        }
        if input.ends_with(&['='])
        {
            l = input[..input.len() - 1].iter().collect::<String>();
            match l.as_str()
            {
                "color" => println!("{}", options.color),
                "prompt" => println!("{}", options.prompt),
                "rt" => println!("{}", options.real_time_output),
                "sci" | "scientific" => println!("{}", options.sci),
                "debug" => println!("{}", options.debug),
                "line" => println!("{}", options.lines),
                "polar" => println!("{}", options.polar),
                "frac" => println!("{}", options.frac),
                "multi" => println!("{}", options.multi),
                "tabbed" => println!("{}", options.tabbed),
                "comma" => println!("{}", options.comma),
                "point" => println!("{}", options.point_style),
                "base" => println!("{}", options.base),
                "decimal" | "deci" | "decimals" => println!("{}", options.decimal_places),
                "prec" | "precision" => println!("{}", options.prec),
                "xr" => println!("{},{}", options.xr[0], options.xr[1]),
                "yr" => println!("{},{}", options.yr[0], options.yr[1]),
                "zr" => println!("{},{}", options.zr[0], options.zr[1]),
                "frac_iter" => println!("{}", options.frac_iter),
                "2d" => println!("{}", options.samples_2d),
                "3d" => println!("{}", options.samples_3d),
                _ =>
                {
                    for i in match get_func(&input_var(&l, &vars, None, options), options)
                    {
                        Ok(n) => n,
                        Err(_) => continue,
                    }
                    {
                        match i
                        {
                            Num(n) =>
                            {
                                let n = get_output(&options, &n);
                                print!(
                                    "{}{}{}",
                                    n.0,
                                    n.1,
                                    if options.color { "\x1b[0m" } else { "" }
                                )
                            }
                            Vector(n) =>
                            {
                                let mut str = String::new();
                                let mut num;
                                for i in n
                                {
                                    num = get_output(&options, &i);
                                    str.push_str(&format!(
                                        "{}{}{},",
                                        num.0,
                                        num.1,
                                        if options.color { "\x1b[0m" } else { "" }
                                    ));
                                }
                                str.pop();
                                print!("{{{}}}", str)
                            }
                            Matrix(n) =>
                            {
                                let mut str = String::new();
                                let mut num;
                                for i in n
                                {
                                    for j in i
                                    {
                                        num = get_output(&options, &j);
                                        str.push_str(&format!(
                                            "{}{}{},",
                                            num.0,
                                            num.1,
                                            if options.color { "\x1b[0m" } else { "" }
                                        ));
                                    }
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
        if input
            .iter()
            .collect::<String>()
            .replace("==", "")
            .replace("!=", "")
            .replace(">=", "")
            .replace("<=", "")
            .contains('=')
        {
            print!("\x1b[J");
            stdout().flush().unwrap();
            let n = input.iter().collect::<String>();
            split = n.splitn(2, '=');
            let s = split.next().unwrap().replace(' ', "");
            l = s;
            r = split.next().unwrap();
            if l.is_empty()
            {
                continue;
            }
            match l.as_str()
            {
                "point" =>
                {
                    if matches!(
                        r,
                        "." | "+"
                            | "x"
                            | "*"
                            | "s"
                            | "S"
                            | "o"
                            | "O"
                            | "t"
                            | "T"
                            | "d"
                            | "D"
                            | "r"
                            | "R"
                    )
                    {
                        options.point_style = r.chars().next().unwrap();
                    }
                    else
                    {
                        println!("Invalid point type");
                    }
                    continue;
                }
                "base" =>
                {
                    options.base = match r.parse::<usize>()
                    {
                        Ok(n) =>
                        {
                            if !(2..=36).contains(&n)
                            {
                                println!("Invalid base");
                                options.base
                            }
                            else
                            {
                                n
                            }
                        }
                        Err(_) =>
                        {
                            println!("Invalid base");
                            options.base
                        }
                    };
                    continue;
                }
                "decimal" | "deci" | "decimals" =>
                {
                    if r == "-1"
                    {
                        options.decimal_places = usize::MAX - 1;
                    }
                    else if r == "-2"
                    {
                        options.decimal_places = usize::MAX;
                    }
                    else
                    {
                        options.decimal_places = match r.parse::<usize>()
                        {
                            Ok(n) => n,
                            Err(_) =>
                            {
                                println!("Invalid decimal");
                                options.decimal_places
                            }
                        };
                    }
                    continue;
                }
                "prec" | "precision" =>
                {
                    options.prec = if r == "0"
                    {
                        println!("Invalid precision");
                        options.prec
                    }
                    else
                    {
                        match r.parse::<u32>()
                        {
                            Ok(n) => n,
                            Err(_) =>
                            {
                                println!("Invalid precision");
                                options.prec
                            }
                        }
                    };
                    if options.allow_vars
                    {
                        v = get_vars(options.prec);
                        for i in &old
                        {
                            for (j, var) in vars.iter_mut().enumerate()
                            {
                                if v.len() > j && i[0] == v[j][0] && i[1] == var[1]
                                {
                                    *var = v[j].clone();
                                }
                            }
                        }
                        old = v;
                    }
                    continue;
                }
                "xr" =>
                {
                    if r.contains(',')
                    {
                        options.xr[0] = match r.split(',').next().unwrap().parse::<f64>()
                        {
                            Ok(n) => n,
                            Err(_) =>
                            {
                                println!("Invalid x range");
                                options.xr[0]
                            }
                        };
                        options.xr[1] = match r.split(',').last().unwrap().parse::<f64>()
                        {
                            Ok(n) => n,
                            Err(_) =>
                            {
                                println!("Invalid x range");
                                options.xr[1]
                            }
                        };
                        continue;
                    }
                }
                "yr" =>
                {
                    if r.contains(',')
                    {
                        options.yr[0] = match r.split(',').next().unwrap().parse::<f64>()
                        {
                            Ok(n) => n,
                            Err(_) =>
                            {
                                println!("Invalid y range");
                                options.yr[0]
                            }
                        };
                        options.yr[1] = match r.split(',').last().unwrap().parse::<f64>()
                        {
                            Ok(n) => n,
                            Err(_) =>
                            {
                                println!("Invalid y range");
                                options.yr[1]
                            }
                        };
                        continue;
                    }
                }
                "zr" =>
                {
                    if r.contains(',')
                    {
                        options.zr[0] = match r.split(',').next().unwrap().parse::<f64>()
                        {
                            Ok(n) => n,
                            Err(_) =>
                            {
                                println!("Invalid z range");
                                options.zr[0]
                            }
                        };
                        options.zr[1] = match r.split(',').last().unwrap().parse::<f64>()
                        {
                            Ok(n) => n,
                            Err(_) =>
                            {
                                println!("Invalid z range");
                                options.zr[1]
                            }
                        };
                        continue;
                    }
                }
                "frac_iter" =>
                {
                    options.frac_iter = match r.parse::<usize>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid frac_iter");
                            options.frac_iter
                        }
                    };
                    continue;
                }
                "2d" =>
                {
                    options.samples_2d = match r.parse::<f64>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid 2d sample size");
                            options.samples_2d
                        }
                    };
                    continue;
                }
                "3d" =>
                {
                    options.samples_3d = match r.parse::<f64>()
                    {
                        Ok(n) => n,
                        Err(_) =>
                        {
                            println!("Invalid 3d sample size");
                            options.samples_3d
                        }
                    };
                    continue;
                }
                _ =>
                {}
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
            continue;
        }
        else if input.contains(&'#')
            || can_graph(&input_var(
                &input.iter().collect::<String>(),
                &vars,
                None,
                options,
            ))
        {
            print!("\x1b[G\x1b[K");
            stdout().flush().unwrap();
            inputs = input
                .iter()
                .collect::<String>()
                .split('#')
                .map(String::from)
                .collect();
            funcs = Vec::new();
            for i in inputs.iter_mut()
            {
                if i.is_empty()
                {
                    continue;
                }
                *i = input_var(i, &vars, None, options)
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
            handles.push(graph(
                inputs,
                funcs,
                options,
                options.deg,
                options.prec,
                watch,
            ));
            continue;
        }
    }
}