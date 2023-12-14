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
    misc::{clear, clearln, convert, get_terminal_width, prompt, read_single_char, write},
    options::{arg_opts, commands, file_opts, set_commands},
    parse::get_func,
    print::{print_answer, print_concurrent},
    vars::{get_cli_vars, get_vars, input_var},
    AngleType::Radians,
};
use crossterm::terminal;
use std::{
    env::{args, var},
    fs::{File, OpenOptions},
    io::{stdin, stdout, BufRead, BufReader, IsTerminal, Write},
    thread::JoinHandle,
    time::Instant,
};
//make parse and vars same function
#[derive(Clone)]
pub struct Colors
{
    text: String,
    prompt: String,
    imag: String,
    sci: String,
    brackets: Vec<String>,
    re1col: String,
    re2col: String,
    re3col: String,
    re4col: String,
    re5col: String,
    re6col: String,
    im1col: String,
    im2col: String,
    im3col: String,
    im4col: String,
    im5col: String,
    im6col: String,
}
impl Default for Colors
{
    fn default() -> Self
    {
        Colors {
            text: "\x1b[0m".to_string(),
            prompt: "\x1b[94m".to_string(),
            imag: "\x1b[93m".to_string(),
            sci: "\x1b[92m".to_string(),
            brackets: vec![
                "\x1b[91m".to_string(),
                "\x1b[92m".to_string(),
                "\x1b[93m".to_string(),
                "\x1b[94m".to_string(),
                "\x1b[95m".to_string(),
                "\x1b[96m".to_string(),
            ],
            re1col: "#ff5555".to_string(),
            re2col: "#5555ff".to_string(),
            re3col: "#ff55ff".to_string(),
            re4col: "#55ff55".to_string(),
            re5col: "#55ffff".to_string(),
            re6col: "#ffff55".to_string(),
            im1col: "#aa0000".to_string(),
            im2col: "#0000aa".to_string(),
            im3col: "#aa00aa".to_string(),
            im4col: "#00aa00".to_string(),
            im5col: "#00aaaa".to_string(),
            im6col: "#aaaa00".to_string(),
        }
    }
}
#[derive(Copy, Clone, PartialEq)]
pub enum AngleType
{
    Radians,
    Degrees,
    Gradians,
}
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
    graph: bool,
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
            multi: true,
            tabbed: false,
            allow_vars: true,
            small_e: false,
            debug: false,
            depth: false,
            graph: true,
        }
    }
}
fn main()
{
    let mut colors = Colors::default();
    let mut options = Options::default();
    let mut args = args().collect::<Vec<String>>();
    {
        #[cfg(unix)]
        let file_path = &(var("HOME").unwrap() + "/.config/kalc.config");
        #[cfg(not(unix))]
        let file_path = &format!(
            "C:\\Users\\{}\\AppData\\Roaming\\kalc.config",
            var("USERNAME").unwrap()
        );
        if let Err(s) = file_opts(&mut options, &mut colors, file_path)
        {
            println!("{}", s);
            std::process::exit(1);
        }
        if let Err(s) = arg_opts(&mut options, &mut colors, &mut args)
        {
            println!("{}", s);
            std::process::exit(1);
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
    let mut vars: Vec<[String; 2]> = if options.allow_vars
    {
        if args.is_empty()
        {
            get_vars(options)
        }
        else
        {
            get_cli_vars(options, &args)
        }
    }
    else
    {
        Vec::new()
    };
    let mut old = vars.clone();
    {
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
                    if !l.starts_with('#')
                    {
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
        }
    }
    let mut stdout = stdout();
    let (mut file, mut unmod_lines) = if args.is_empty()
    {
        terminal::enable_raw_mode().unwrap();
        print!(
            "\x1b[G\x1b[K{}{}",
            prompt(options, &colors),
            if options.color { "\x1b[0m" } else { "" }
        );
        stdout.flush().unwrap();
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
        (
            Some(OpenOptions::new().append(true).open(file_path).unwrap()),
            Some(
                BufReader::new(File::open(file_path).unwrap())
                    .lines()
                    .map(|l| l.unwrap())
                    .collect::<Vec<String>>(),
            ),
        )
    }
    else
    {
        options.color = !options.color;
        (None, None)
    };
    let mut watch = None;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let mut cut: Vec<char> = Vec::new();
    'main: loop
    {
        let mut input = Vec::new();
        let mut graphable = false;
        if !args.is_empty()
        {
            if options.debug
            {
                watch = Some(Instant::now());
            }
            input = args.remove(0).chars().collect();
            print_answer(
                &input.iter().collect::<String>(),
                match get_func(
                    &input_var(
                        &input.iter().map(convert).collect::<String>(),
                        vars.clone(),
                        None,
                        &mut Vec::new(),
                        &mut 0,
                        options,
                        0,
                    ),
                    options,
                )
                {
                    Ok(f) => f,
                    Err(s) =>
                    {
                        println!("{}: {}", input.iter().collect::<String>(), s);
                        continue;
                    }
                },
                options,
                &colors,
            );
            if let Some(time) = watch
            {
                print!(" {}", time.elapsed().as_nanos());
            }
            if !can_graph(
                &input_var(
                    &input.iter().collect::<String>(),
                    vars.clone(),
                    None,
                    &mut Vec::new(),
                    &mut 0,
                    options,
                    0,
                ),
                options.graph,
            )
            {
                println!();
            }
            graphable = true;
        }
        else
        {
            if file.is_none()
            {
                for handle in handles
                {
                    handle.join().unwrap();
                }
                break;
            }
            let mut frac = 0;
            let mut current = Vec::new();
            let mut lines = unmod_lines.clone().unwrap();
            let mut i = lines.len();
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
                    '\n' | '\x14' =>
                    {
                        end = start + get_terminal_width() - if options.prompt { 3 } else { 1 };
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        if !options.real_time_output && !input.is_empty()
                        {
                            frac = print_concurrent(
                                &input,
                                &last,
                                &vars.clone(),
                                options,
                                &colors,
                                start,
                                end,
                            );
                        }
                        if !can_graph(
                            &input_var(
                                &input.iter().collect::<String>(),
                                vars.clone(),
                                None,
                                &mut Vec::new(),
                                &mut 0,
                                options,
                                0,
                            ),
                            options.graph,
                        )
                        {
                            println!();
                        }
                        if !input.is_empty()
                        {
                            println!("{}", "\n".repeat(frac));
                        }
                        if c == '\x14'
                        {
                            if input.is_empty()
                            {
                                print!("\x1b[A")
                            }
                            print!("\x1b[G\x1b[J");
                            terminal::disable_raw_mode().unwrap();
                            std::process::exit(0);
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
                        if i == lines.len()
                        {
                            current = input.clone();
                        }
                        else
                        {
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        frac = if options.real_time_output
                        {
                            print_concurrent(&input, &last, &vars, options, &colors, start, end)
                        }
                        else
                        {
                            clear(&input, start, end, options, &colors);
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
                        if i == lines.len()
                        {
                            current = input.clone();
                        }
                        else
                        {
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        frac = if options.real_time_output
                        {
                            print_concurrent(&input, &last, &vars, options, &colors, start, end)
                        }
                        else
                        {
                            clear(&input, start, end, options, &colors);
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
                            frac = print_concurrent(
                                &input, &last, &vars, options, &colors, start, end,
                            );
                        }
                        else
                        {
                            clear(&input, start, end, options, &colors);
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
                            frac = print_concurrent(
                                &input, &last, &vars, options, &colors, start, end,
                            );
                        }
                        else
                        {
                            clear(&input, start, end, options, &colors);
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
                            frac = print_concurrent(
                                &input, &last, &vars, options, &colors, start, end,
                            );
                        }
                        else
                        {
                            clear(&input, start, end, options, &colors);
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
                            frac = print_concurrent(
                                &input, &last, &vars, options, &colors, start, end,
                            );
                        }
                        else
                        {
                            clear(&input, start, end, options, &colors);
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
                                frac = print_concurrent(
                                    &input, &last, &vars, options, &colors, start, end,
                                );
                            }
                            else
                            {
                                clear(&input, start, end, options, &colors);
                            }
                            print!("{}", "\x08".repeat(end - start - (placement - start)));
                        }
                    }
                    '\x15' =>
                    {
                        //ctrl+l
                        if options.real_time_output && !input.is_empty()
                        {
                            frac = print_concurrent(
                                &input, &last, &vars, options, &colors, start, end,
                            );
                        }
                        else
                        {
                            clear(&input, start, end, options, &colors);
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
                            frac = print_concurrent(
                                &input, &last, &vars, options, &colors, start, end,
                            );
                        }
                        else
                        {
                            clear(&input, start, end, options, &colors);
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
                            frac = print_concurrent(
                                &input, &last, &vars, options, &colors, start, end,
                            );
                        }
                        else
                        {
                            clear(&input, start, end, options, &colors);
                        }
                    }
                    '\x1E' =>
                    {
                        //down history
                        i += 1;
                        if i >= lines.len()
                        {
                            i = lines.len();
                            input = current.clone();
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
                            frac = print_concurrent(
                                &input, &last, &vars, options, &colors, start, end,
                            );
                        }
                        else
                        {
                            clear(&input, start, end, options, &colors);
                        }
                    }
                    '\x1B' =>
                    {
                        //left
                        if placement - start == 0 && placement != 0 && start != 0
                        {
                            start -= 1;
                            placement -= 1;
                            end = start + get_terminal_width() - if options.prompt { 3 } else { 1 };
                            if end > input.len()
                            {
                                end = input.len()
                            }
                            clearln(&input, start, end, options, &colors);
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
                        //right
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
                            clearln(&input, start, end, options, &colors)
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
                                clearln(&input, start, end, options, &colors);
                                print!("{}", "\x08".repeat(end - start - (placement - start)));
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
                                clearln(&input, start, end, options, &colors)
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
                        if i == lines.len()
                        {
                            current = input.clone();
                        }
                        else
                        {
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        if options.real_time_output
                        {
                            frac = print_concurrent(
                                &input, &last, &vars, options, &colors, start, end,
                            );
                        }
                        else
                        {
                            clear(&input, start, end, options, &colors)
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
                print!(
                    "\x1b[G\x1b[K{}{}",
                    prompt(options, &colors),
                    if options.color { "\x1b[0m" } else { "" }
                );
                stdout.flush().unwrap();
                continue;
            }
            commands(
                &mut options,
                &colors,
                &mut watch,
                &mut vars,
                &mut old,
                &lines,
                &input,
                &mut stdout,
            );
            print!(
                "\x1b[G\x1b[K{}{}",
                prompt(options, &colors),
                if options.color { "\x1b[0m" } else { "" }
            );
            stdout.flush().unwrap();
            write(
                &input
                    .iter()
                    .collect::<String>()
                    .replace("small_e", "smalle")
                    .replace("frac_iter", "fraciter")
                    .replace('_', &format!("({})", last.iter().collect::<String>()))
                    .replace("smalle", "small_e")
                    .replace("fraciter", "frac_iter"),
                file.as_mut().unwrap(),
                unmod_lines.as_mut().unwrap(),
            );
            if !input.ends_with(&['='])
            {
                if input
                    .iter()
                    .collect::<String>()
                    .replace("==", "")
                    .replace("!=", "")
                    .replace(">=", "")
                    .replace("<=", "")
                    .contains('=')
                {
                    let n = input.iter().collect::<String>();
                    let mut split = n.splitn(2, '=');
                    let s = split.next().unwrap().replace(' ', "");
                    let l = s;
                    let r = split.next().unwrap();
                    if l.is_empty()
                        || match set_commands(&mut options, &mut colors, &mut vars, &mut old, &l, r)
                        {
                            Err(s) =>
                            {
                                if !s.is_empty()
                                {
                                    if s == " "
                                    {
                                        print!(
                                            "\x1b[G\x1b[Kempty input\n\x1b[G\x1b[K{}{}",
                                            prompt(options, &colors),
                                            if options.color { "\x1b[0m" } else { "" },
                                        );
                                        stdout.flush().unwrap()
                                    }
                                    else
                                    {
                                        print!(
                                            "\x1b[G\x1b[K{}\n\x1b[G\x1b[K{}{}",
                                            s,
                                            prompt(options, &colors),
                                            if options.color { "\x1b[0m" } else { "" },
                                        );
                                        stdout.flush().unwrap()
                                    }
                                }
                                true
                            }
                            _ => false,
                        }
                    {
                        continue;
                    }
                    if l.contains('(')
                    {
                        let s = l.split('(').next().iter().copied().collect::<String>() + "(";
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
                            && v[0].contains('(') == l.contains('(')
                            && v[0].chars().filter(|c| c == &',').count()
                                == l.chars().filter(|c| c == &',').count()
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
        }
        if graphable
            && !input.iter().collect::<String>().starts_with("history")
            && (input.contains(&'#')
                || can_graph(
                    &input_var(
                        &input.iter().collect::<String>(),
                        vars.clone(),
                        None,
                        &mut Vec::new(),
                        &mut 0,
                        options,
                        0,
                    ),
                    options.graph,
                ))
        {
            let mut inputs: Vec<String> = input
                .iter()
                .collect::<String>()
                .split('#')
                .map(String::from)
                .collect();
            let unmod = inputs.clone();
            let mut funcs = Vec::new();
            if inputs.len() < 7
            {
                for i in inputs.iter_mut()
                {
                    if i.is_empty()
                    {
                        continue;
                    }
                    *i = input_var(i, vars.clone(), None, &mut Vec::new(), &mut 0, options, 0);
                    funcs.push(match get_func(i, options)
                    {
                        Ok(f) => f,
                        _ => continue 'main,
                    });
                }
                handles.push(graph(inputs, unmod, funcs, options, watch, colors.clone()));
            }
        }
    }
}