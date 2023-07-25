mod complex;
mod fraction;
mod graph;
mod math;
mod options;
mod parse;
mod print;
#[cfg(test)]
mod tests;
use crate::{
    complex::NumStr::{Matrix, Num, Str, Vector},
    graph::graph,
    math::do_math,
    options::{
        arg_opts, file_opts, AngleType,
        AngleType::{Degrees, Gradians, Radians},
    },
    parse::{get_func, get_vars, input_var},
    print::{get_output, print_answer, print_concurrent},
};
use console::{Key, Term};
#[cfg(unix)]
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::{
    env::{args, var},
    fs::{File, OpenOptions},
    io::{stdin, stdout, BufRead, BufReader, IsTerminal, Write},
    thread::JoinHandle,
};
#[cfg(not(unix))]
use term_size::dimensions;
// allow f16/f32/f64/f128 instead of arbitary precision for performance reasons
// gui support (via egui prob)
// support units
// support plus-minus via a 2 vector
// something something digraphs
// fix ³/4 not working
// have '_' go up the stack more
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
            split = i.split('=');
            vars.push([
                split.next().unwrap().to_string(),
                split.next().unwrap().to_string(),
            ]);
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
        mut end,
        mut placement,
    );
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
            if !(input.is_empty()
                || input.contains(&'#')
                || input_var(&input.iter().collect::<String>(), &vars, None, options)
                    .replace("exp", "")
                    .replace("max", "")
                    .replace("}x{", "")
                    .replace("]x[", "")
                    .contains('x')
                || input_var(&input.iter().collect::<String>(), &vars, None, options).contains('y')
                || input_var(&input.iter().collect::<String>(), &vars, None, options)
                    .replace("zeta", "")
                    .replace("normalize", "")
                    .contains('z')
                || input
                    .iter()
                    .collect::<String>()
                    .replace("==", "")
                    .replace("!=", "")
                    .replace(">=", "")
                    .replace("<=", "")
                    .contains('='))
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
                print!("{}> \x1b[0m", if options.color { "\x1b[94m" } else { "" });
                stdout().flush().unwrap();
            }
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
            'outer: loop
            {
                c = read_single_char();
                if c == '\x1A'
                {
                    c = match read_single_char()
                    {
                        'a' => 'α',
                        'A' => 'Α',
                        'b' => 'β',
                        'B' => 'Β',
                        'c' => 'χ',
                        'C' => 'Χ',
                        'd' => 'Δ',
                        'D' => 'δ',
                        'e' => 'ε',
                        'E' => 'Ε',
                        'f' => 'φ',
                        'F' => 'Φ',
                        'g' => 'γ',
                        'G' => 'Γ',
                        'h' => 'η',
                        'H' => 'Η',
                        'i' => 'ι',
                        'I' => 'Ι',
                        'k' => 'κ',
                        'Κ' => 'Κ',
                        'l' => 'λ',
                        'L' => 'Λ',
                        'm' => 'μ',
                        'M' => 'Μ',
                        'n' => 'ν',
                        'Ν' => 'Ν',
                        'o' => 'ο',
                        'O' => 'Ο',
                        'p' => 'π',
                        'P' => 'Π',
                        'q' => 'θ',
                        'Q' => 'Θ',
                        'r' => 'ρ',
                        'R' => 'Ρ',
                        's' => 'σ',
                        'S' => 'Σ',
                        't' => 'τ',
                        'T' => 'Τ',
                        'u' => 'υ',
                        'U' => 'Υ',
                        'w' => 'ω',
                        'W' => 'Ω',
                        'y' => 'ψ',
                        'Y' => 'Ψ',
                        'x' => 'ξ',
                        'X' => 'Ξ',
                        'z' => 'ζ',
                        'Z' => 'Ζ',
                        _ => continue,
                    }
                }
                if options.debug
                {
                    watch = Some(std::time::Instant::now());
                }
                match c
                {
                    '\n' =>
                    {
                        end = start + get_terminal_width() - if options.prompt { 3 } else { 0 };
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        if !options.real_time_output
                        {
                            frac = print_concurrent(
                                &input,
                                &input_var(&input.iter().collect::<String>(), &vars, None, options)
                                    .replace(
                                        '_',
                                        &format!("({})", last.iter().collect::<String>()),
                                    ),
                                options,
                                start,
                                end,
                            );
                        }
                        if !(input.is_empty()
                            || input.contains(&'#')
                            || input_var(&input.iter().collect::<String>(), &vars, None, options)
                                .replace("exp", "")
                                .replace("max", "")
                                .replace("}x{", "")
                                .replace("]x[", "")
                                .contains('x')
                            || input_var(&input.iter().collect::<String>(), &vars, None, options)
                                .contains('y')
                            || input_var(&input.iter().collect::<String>(), &vars, None, options)
                                .replace("zeta", "")
                                .replace("normalize", "")
                                .contains('z')
                            || input
                                .iter()
                                .collect::<String>()
                                .replace("==", "")
                                .replace("!=", "")
                                .replace(">=", "")
                                .replace("<=", "")
                                .contains('='))
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
                                print!(
                                    "\x1B[0J\x1B[2K\x1B[1G{}",
                                    if options.prompt
                                    {
                                        if options.color
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
                                    }
                                );
                                stdout().flush().unwrap();
                            }
                            continue;
                        }
                        placement -= 1;
                        input.remove(placement);
                        end = start + get_terminal_width() - if options.prompt { 3 } else { 0 };
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
                        frac = if input.is_empty()
                        {
                            0
                        }
                        else if options.real_time_output
                        {
                            print_concurrent(
                                &input,
                                &input_var(&input.iter().collect::<String>(), &vars, None, options)
                                    .replace(
                                        '_',
                                        &format!("({})", last.iter().collect::<String>()),
                                    ),
                                options,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            print!(
                                "\x1B[2K\x1B[1G{}{}\x1b[0m",
                                if options.prompt
                                {
                                    if options.color
                                    {
                                        "\x1b[94m> \x1b[96m"
                                    }
                                    else
                                    {
                                        "> "
                                    }
                                }
                                else if options.color
                                {
                                    "\x1b[96m"
                                }
                                else
                                {
                                    ""
                                },
                                &input[start..end].iter().collect::<String>()
                            );
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
                        if placement == 0 && input.is_empty()
                        {
                            print!(
                                "\x1B[0J\x1B[2K\x1B[1G{}",
                                if options.prompt
                                {
                                    if options.color
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
                                }
                            );
                        }
                    }
                    '\x1D' =>
                    {
                        // up history
                        i -= if i > 0 { 1 } else { 0 };
                        input = lines[i as usize].clone().chars().collect::<Vec<char>>();
                        placement = input.len();
                        end = input.len();
                        start = if get_terminal_width() - if options.prompt { 3 } else { 0 }
                            > input.len()
                        {
                            0
                        }
                        else
                        {
                            input.len()
                                - (get_terminal_width() - if options.prompt { 3 } else { 0 })
                        };
                        if options.real_time_output
                        {
                            frac = print_concurrent(
                                &input,
                                &input_var(&input.iter().collect::<String>(), &vars, None, options)
                                    .replace(
                                        '_',
                                        &format!("({})", last.iter().collect::<String>()),
                                    ),
                                options,
                                start,
                                end,
                            );
                        }
                        else
                        {
                            print!(
                                "\x1B[0J\x1B[2K\x1B[1G{}{}\x1b[0m",
                                if options.prompt
                                {
                                    if options.color
                                    {
                                        "\x1b[94m> \x1b[96m"
                                    }
                                    else
                                    {
                                        "> "
                                    }
                                }
                                else if options.color
                                {
                                    "\x1b[96m"
                                }
                                else
                                {
                                    ""
                                },
                                &input[start..].iter().collect::<String>()
                            );
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
                                       if options.prompt
                                       {
                                           if options.color
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
                            input = lines[i as usize].clone().chars().collect::<Vec<char>>();
                        }
                        placement = input.len();
                        end = input.len();
                        start = if get_terminal_width() - if options.prompt { 3 } else { 0 }
                            > input.len()
                        {
                            0
                        }
                        else
                        {
                            input.len()
                                - (get_terminal_width() - if options.prompt { 3 } else { 0 })
                        };
                        if options.real_time_output
                        {
                            frac = print_concurrent(
                                &input,
                                &input_var(&input.iter().collect::<String>(), &vars, None, options)
                                    .replace(
                                        '_',
                                        &format!("({})", last.iter().collect::<String>()),
                                    ),
                                options,
                                start,
                                end,
                            );
                        }
                        else
                        {
                            print!(
                                "\x1B[2K\x1B[1G{}{}\x1b[0m",
                                if options.prompt
                                {
                                    if options.color
                                    {
                                        "\x1b[94m> \x1b[96m"
                                    }
                                    else
                                    {
                                        "> "
                                    }
                                }
                                else if options.color
                                {
                                    "\x1b[96m"
                                }
                                else
                                {
                                    ""
                                },
                                &input[start..].iter().collect::<String>()
                            );
                        }
                    }
                    '\x1B' =>
                    {
                        // go left
                        if placement - start == 0 && placement != 0 && start != 0
                        {
                            start -= 1;
                            placement -= 1;
                            end = start + get_terminal_width() - if options.prompt { 3 } else { 0 };
                            if end > input.len()
                            {
                                end = input.len()
                            }
                            print!(
                                "\x1B[2K\x1B[1G{}{}\x1b[0m{}",
                                if options.prompt
                                {
                                    if options.color
                                    {
                                        "\x1b[94m> \x1b[96m"
                                    }
                                    else
                                    {
                                        "> "
                                    }
                                }
                                else if options.color
                                {
                                    "\x1b[96m"
                                }
                                else
                                {
                                    ""
                                },
                                &input[start..end].iter().collect::<String>(),
                                "\x08".repeat(end - start - (placement - start))
                            );
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
                        end = start + get_terminal_width() - if options.prompt { 3 } else { 0 };
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        if placement == end && end != input.len()
                        {
                            start += 1;
                            placement += 1;
                            print!(
                                "\x1B[2K\x1B[1G{}{}\x1b[0m",
                                if options.prompt
                                {
                                    if options.color
                                    {
                                        "\x1b[94m> \x1b[96m"
                                    }
                                    else
                                    {
                                        "> "
                                    }
                                }
                                else if options.color
                                {
                                    "\x1b[96m"
                                }
                                else
                                {
                                    ""
                                },
                                &input[start..end + 1].iter().collect::<String>()
                            );
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
                        //convert_str(&mut input, c, &mut placement, &mut exp);
                        input.insert(placement, c);
                        placement += 1;
                        end = start + get_terminal_width() - if options.prompt { 3 } else { 0 } + 1;
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
                            lines[i as usize] = input.clone().iter().collect::<String>();
                        }
                        if options.real_time_output
                        {
                            frac = print_concurrent(
                                &input,
                                &input_var(&input.iter().collect::<String>(), &vars, None, options)
                                    .replace(
                                        '_',
                                        &format!("({})", last.iter().collect::<String>()),
                                    ),
                                options,
                                start,
                                end,
                            );
                        }
                        else
                        {
                            print!(
                                "\x1B[0J\x1B[2K\x1B[1G{}{}\x1b[0m",
                                if options.prompt
                                {
                                    if options.color
                                    {
                                        "\x1b[94m> \x1b[96m"
                                    }
                                    else
                                    {
                                        "> "
                                    }
                                }
                                else if options.color
                                {
                                    "\x1b[96m"
                                }
                                else
                                {
                                    ""
                                },
                                &input[start..end].iter().collect::<String>()
                            );
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
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.color = !options.color;
                }
                "prompt" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.prompt = !options.prompt;
                }
                "deg" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.deg = Degrees;
                }
                "rad" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.deg = Radians;
                }
                "grad" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.deg = Gradians;
                }
                "rt" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.real_time_output = !options.real_time_output;
                }
                "tau" => options.tau = true,
                "pi" => options.tau = false,
                "small_e" => options.small_e = !options.small_e,
                "sci" | "scientific" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.sci = !options.sci;
                }
                "clear" =>
                {
                    print!("\x1B[2J\x1B[1;1H");
                    stdout().flush().unwrap();
                }
                "debug" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.debug = !options.debug;
                    watch = None;
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
                    options.lines = !options.lines;
                }
                "polar" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.polar = !options.polar;
                }
                "frac" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.frac = !options.frac;
                }
                "multi" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.multi = !options.multi;
                }
                "tabbed" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.tabbed = !options.tabbed;
                }
                "comma" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    options.comma = !options.comma;
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
                            n = get_output(
                                &options,
                                &do_math(
                                    get_func(
                                        &input_var(&v[1], &vars, Some(&v[0]), options),
                                        options,
                                    )
                                    .unwrap(),
                                    options.deg,
                                    options.prec,
                                )
                                .unwrap()
                                .num()
                                .unwrap(),
                            );
                            println!("{}={}{}", v[0], n.0, n.1);
                        }
                    }
                }
                "lvars" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    for v in vars.iter()
                    {
                        println!("{}={}", v[0], v[1]);
                    }
                }
                "version" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                    continue;
                }
                "exit" | "quit" | "break" =>
                {
                    break;
                }
                _ =>
                {
                    let n = input.iter().collect::<String>();
                    split = n.splitn(2, ' ');
                    if split.next().unwrap() == "history"
                    {
                        print!("\x1b[A\x1B[2K\x1B[1G");
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
                }
            }
            write(&input.iter().collect::<String>(), &mut file, &unmod_lines);
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
            print!("\x1B[0J");
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
                    if r == "."
                        || r == "+"
                        || r == "x"
                        || r == "*"
                        || r == "s"
                        || r == "S"
                        || r == "o"
                        || r == "O"
                        || r == "t"
                        || r == "T"
                        || r == "d"
                        || r == "D"
                        || r == "r"
                        || r == "R"
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
        else if input.contains(&'#')
            || input_var(&input.iter().collect::<String>(), &vars, None, options)
                .replace("exp", "")
                .replace("max", "")
                .replace("}x{", "")
                .replace("]x[", "")
                .contains('x')
            || input_var(&input.iter().collect::<String>(), &vars, None, options)
                .replace("zeta", "")
                .replace("normalize", "")
                .contains('z')
        {
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            inputs = input
                .iter()
                .collect::<String>()
                .split('#')
                .map(String::from)
                .collect();
            funcs = Vec::new();
            for i in &inputs
            {
                if i.is_empty()
                {
                    continue;
                }
                funcs.push(
                    match get_func(
                        &input_var(i, &vars, None, options)
                            .replace("zeta", "##ta##")
                            .replace("normalize", "##ma##")
                            .replace('z', "(x+y*i)")
                            .replace("##ta##", "zeta")
                            .replace("##ma##", "normalize"),
                        options,
                    )
                    {
                        Ok(f) => f,
                        _ => continue 'main,
                    },
                );
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
#[cfg(unix)]
fn get_terminal_width() -> usize
{
    unsafe {
        let mut size: winsize = std::mem::zeroed();
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
fn convert(c: &char) -> char
{
    let valid_chars = [
        '+', '^', '(', ')', '.', '=', ',', '#', '|', '&', '!', '%', '_', '<', '>', ' ', '[', ']',
        '{', '}', '√', '∛', '¼', '½', '¾', '⅐', '⅑', '⅒', '⅓', '⅔', '⅕', '⅖', '⅗', '⅘', '⅙', '⅚',
        '⁹', '⁸', '⁷', '⁶', '⁵', '⁴', '³', '²', '¹', '⁰', '⅛', '⅜', '⅝', '⅞', '⅟', '↉', '⁻', 'ⁱ',
    ];
    match c
    {
        c if c.is_alphanumeric() || valid_chars.contains(c) => *c,
        'ⲡ' | '𝜋' | '𝛑' | '𝝿' | '𝞹' | '𝝅' | 'ℼ' | 'π' => 'π',
        'ⲧ' | '𝛕' | '𝜏' | '𝝉' | '𝞃' | '𝞽' | 'τ' => 'τ',
        '∗' | '∙' | '*' | '·' | '⋅' => '*',
        '∕' | '⁄' | '/' => '/',
        '−' | '-' => '-',
        '₀' => '0',
        '₁' => '1',
        '₂' => '2',
        '₃' => '3',
        '₄' => '4',
        '₅' => '5',
        '₆' => '6',
        '₇' => '7',
        '₈' => '8',
        '₉' => '9',
        _ => '\0',
    }
}
fn read_single_char() -> char
{
    let term = Term::stdout();
    let key = term.read_key().unwrap();
    match key
    {
        Key::Char(c) => convert(&c),
        Key::Enter => '\n',
        Key::Backspace => '\x08',
        Key::ArrowLeft => '\x1B',
        Key::ArrowRight => '\x1C',
        Key::ArrowUp => '\x1D',
        Key::ArrowDown => '\x1E',
        Key::Escape => '\x1A',
        _ => '\0',
    }
}
fn write(input: &str, file: &mut File, lines: &Vec<String>)
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
--2d=[num] number of points to graph in 2D\n\
--3d=[num] number of points to graph in 3D\n\
--xr=[min],[max] x range for graphing\n\
--yr=[min],[max] y range for graphing\n\
--zr=[min],[max] z range for graphing\n\
--point [char] point style for graphing\n\
--sci toggles scientific notation\n\
--base=[num] sets the number base (2,8,16)\n\
--prompt toggles the prompt\n\
--color toggles color\n\
--comma toggles comma seperation\n\
--vars toggles default variables\n\
--line toggles line graphing\n\
--rt toggles real time printing\n\
--polar toggles displaying polar vectors\n\
--frac toggles fraction display\n\
--frac_iter=[num] how many iterations to check for fractions\n\
--prec=[num] sets the precision\n\
--deci=[num] sets how many decimals to display, -1 for length of terminal, -2 for maximum decimal places, may need to up precision for more decimals\n\
--def ignores config file\n\
--multi toggles multi line display for matrixes\n\
--tabbed toggles tabbed display for matrixes\n\
--debug displays computation time in nanoseconds\n\
--small_e use small e notation, like 5e2=5*10^2, instead of capital 'E' for scientific notation. only works with a number before and number or '-' sign after the 'e' otherwise assumes euler number\n\n\
- flags can be executed in runtime just without the dashes\n\
- Type \"exit\" to exit the program\n\
- Type \"clear\" to clear the screen\n\
- Type \"history [arg]\" to see the history, arg indexes it if specified\n\
- Type \"vars\" to list all variables\n\
- Type \"lvars\" to list all variables without equating them\n\
- Type \"_\" to use the previous answer\n\
- Type \"a={{expr}}\" to define a variable\n\
- Type \"f(x)=...\" to define a function\n\
- Type \"f(x,y,z...)=...\" to define a multi variable function\n\
- Type \"...=\" display parsed input, show values of stuff like xr/deci/prec etc\n\
- Type \"f...=null\" to delete a function or variable\n\
- Type \"{{x,y,z...}}\" to define a cartesian vector\n\
- Type \"[radius,theta,phi]\" to define a polar vector (same as car{{vec}})\n\
- Type \"{{vec}}#\" to graph a vector\n\
- Type \"{{mat}}#\" to graph a matrix\n\
- Type \"number#\" to graph a complex number\n\
- Type \"{{{{a,b,c}},{{d,e,f}},{{g,h,i}}}}\" to define a 3x3 matrix\n\n\
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
- ln, log(base,num), root(base,exp), sum(func,var,start,end), prod(func,var,start,end) (start and end are rounded to integers)\n\
- abs, sgn, arg\n\
- ceil, floor, round, int, frac\n\
- fact(real), subfact(natural)\n\
- sinc, cis, exp\n\
- zeta, gamma, erf, erfc, digamma, ai, binomial/bi (all real only)\n\
- deg(to_degrees), rad(to_radians), grad(to_gradians) (all real only)\n\
- re, im, max(x,y), min(x,y)\n\n\
Vector operations/functions:\n\
- *,/,+,-,^\n\
- dot({{vec1}},{{vec2}}), cross({{vec1}},{{vec2}}), proj/project({{vec1}},{{vec2}})\n\
- angle({{vec1}},{{vec2}})\n\
- norm, normalize\n\
- abs, len\n\
- part({{vec}},col)\n\
- convert to polar: pol{{vec}} outputs (radius, theta, phi)\n\
- convert to cartesian: car{{vec}} outputs (x, y, z)\n\
- other functions are applied like sqrt{{2,4}}={{sqrt(2),sqrt(4)}}\n\n\
Matrix operations/functions:\n\
- *,/,+,-,^\n\
- trace/tr, determinant/det, inverse/inv\n\
- transpose/trans, adjugate/adj, cofactor/cof, minor\n\
- part({{mat}},col,row)\n\
- abs, norm\n\
- len, wid\n\
- rotate(theta) produces a rotational matrix\n\
- other functions are applied like sqrt{{{{2,4}},{{5,6}}}}={{{{sqrt(2),sqrt(4)}},{{sqrt(5),sqrt(6)}}}}\n\n\
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
- tau: tau, 6.2831853071~\n\n\
Digraph:\n\
hit escape then a letter\n\
a=>α, A=>Α, b=>β, B=>Β, c=>χ, C=>Χ, d=>Δ, D=>δ,\n\
e=>ε, E=>Ε, f=>φ, F=>Φ, g=>γ, G=>Γ, h=>η, H=>Η,\n\
i=>ι, I=>Ι, k=>κ, Κ=>Κ, l=>λ, L=>Λ, m=>μ, M=>Μ,\n\
n=>ν, Ν=>Ν, o=>ο, O=>Ο, p=>π, P=>Π, q=>θ, Q=>Θ,\n\
r=>ρ, R=>Ρ, s=>σ, S=>Σ, t=>τ, T=>Τ, u=>υ, U=>Υ,\n\
w=>ω, W=>Ω, y=>ψ, Y=>Ψ, x=>ξ, X=>Ξ, z=>ζ, Z=>Ζ,"
    );
}