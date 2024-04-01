mod complex;
mod fraction;
mod functions;
mod graph;
mod help;
mod load_vars;
mod math;
mod misc;
mod options;
mod parse;
mod print;
#[cfg(test)]
mod tests;
mod units;
use crate::{
    complex::NumStr,
    graph::graph,
    load_vars::{add_var, get_cli_vars, get_file_vars, get_vars, set_commands_or_vars},
    math::do_math,
    misc::{
        clear, clearln, convert, get_terminal_dimensions, handle_err, prompt, read_single_char,
        write,
    },
    options::{arg_opts, commands, file_opts},
    parse::input_var,
    print::{print_answer, print_concurrent},
    AngleType::Radians,
    Notation::Normal,
};
use crossterm::{
    cursor::{DisableBlinking, EnableBlinking},
    execute, terminal,
};
use rug::Complex;
use std::{
    env::{args, var},
    fs::{File, OpenOptions},
    io::{stdin, stdout, BufRead, BufReader, IsTerminal, Write},
    thread::JoinHandle,
    time::Instant,
};
//rpn
//domain coloring
//matrix exponentiation
//surface area of a 3d curve
//maybe nth area, make -nth go into the other function
//stop vectors or matrixes disipearing graphically by making them have points instead of lines
//do as much as you can before graphing, make '=' at end show unsimplified, and no = show simplififed
//allow setting x and y window dimensions of gnuplot, defaulting to what happens now
//infinitely long greater/lesser than chains
//have unit powers be fractional, maybe 2 usizes
//add option to print units in simplified form instead of just in base si units, like N instead of kg m s^-2
//compress units if makes sense, ie m s^-2 -> m/s^2
//allow changing the default unit for all, like angles
//maybe add exchange rates via something like https://www.floatrates.com/daily/usd.json
#[derive(Clone)]
pub struct Variable
{
    name: Vec<char>,
    parsed: Vec<NumStr>,
    unparsed: String,
    funcvars: Vec<(String, Vec<NumStr>)>,
}
#[derive(Clone, PartialEq, Copy)]
pub struct Units
{
    second: f64,
    meter: f64,
    kilogram: f64,
    ampere: f64,
    kelvin: f64,
    mole: f64,
    candela: f64,
    angle: f64,
    byte: f64,
}
#[derive(Clone, PartialEq)]
pub struct Number
{
    number: Complex,
    units: Option<Units>,
}
#[derive(Clone)]
pub struct Colors
{
    text: String,
    prompt: String,
    imag: String,
    sci: String,
    units: String,
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
    label: (String, String, String),
}
impl Default for Colors
{
    fn default() -> Self
    {
        Self {
            text: "\x1b[0m".to_string(),
            prompt: "\x1b[94m".to_string(),
            imag: "\x1b[93m".to_string(),
            sci: "\x1b[92m".to_string(),
            units: "\x1b[96m".to_string(),
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
            label: ("x".to_string(), "y".to_string(), "z".to_string()),
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
#[derive(Copy, Clone, PartialEq)]
pub enum Notation
{
    Normal,
    Scientific,
    LargeEngineering,
    SmallEngineering,
}
#[derive(Clone, Copy)]
pub struct Options
{
    notation: Notation,
    angle: AngleType,
    base: (i32, i32),
    ticks: f64,
    onaxis: bool,
    polar: bool,
    frac: bool,
    real_time_output: bool,
    decimal_places: usize,
    color: bool,
    prompt: bool,
    comma: bool,
    prec: u32,
    graph_prec: u32,
    xr: (f64, f64),
    yr: (f64, f64),
    zr: (f64, f64),
    vxr: (f64, f64),
    vyr: (f64, f64),
    vzr: (f64, f64),
    samples_2d: usize,
    samples_3d: (usize, usize),
    point_style: char,
    lines: bool,
    multi: bool,
    tabbed: bool,
    allow_vars: bool,
    debug: bool,
    depth: bool,
    flat: bool,
    graph: bool,
    slowcheck: u128,
    interactive: bool,
    surface: bool,
    scale_graph: bool,
    stay_interactive: bool,
    graph_cli: bool,
    units: bool,
    window_size: (usize, usize),
}
impl Default for Options
{
    fn default() -> Self
    {
        Self {
            notation: Normal,
            angle: Radians,
            base: (10, 10),
            ticks: 20.0,
            onaxis: true,
            polar: false,
            frac: true,
            real_time_output: true,
            decimal_places: 12,
            color: true,
            prompt: true,
            comma: false,
            prec: 512,
            graph_prec: 128,
            xr: (-10.0, 10.0),
            yr: (-10.0, 10.0),
            zr: (-10.0, 10.0),
            vxr: (0.0, 0.0),
            vyr: (0.0, 0.0),
            vzr: (0.0, 0.0),
            samples_2d: 10000,
            samples_3d: (250, 250),
            point_style: '.',
            lines: false,
            multi: true,
            tabbed: false,
            allow_vars: true,
            debug: false,
            depth: false,
            flat: false,
            graph: true,
            slowcheck: 300,
            interactive: true,
            surface: false,
            scale_graph: true,
            stay_interactive: false,
            graph_cli: false,
            units: true,
            window_size: (0, 0),
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
        let mut lines = Vec::new();
        for line in stdin().lock().lines()
        {
            if !line.as_ref().unwrap().is_empty()
            {
                if let Ok(line) = line
                {
                    if !line.starts_with('#') && !line.is_empty()
                    {
                        lines.push(line);
                    }
                }
            }
        }
        args.splice(0..0, lines);
    }
    options.interactive = args.is_empty();
    let mut stdout = stdout();
    if options.interactive
    {
        terminal::enable_raw_mode().unwrap();
        print!(
            "\x1b[G\x1b[K{}{}",
            prompt(options, &colors),
            if options.color { "\x1b[0m" } else { "" }
        );
        stdout.flush().unwrap();
    }
    for arg in args.iter_mut()
    {
        if (arg.starts_with('\'') && arg.ends_with('\''))
            || (arg.starts_with('\"') && arg.ends_with('\"'))
        {
            arg.remove(0);
            arg.pop();
        }
    }
    #[cfg(unix)]
    let file_path = &(var("HOME").unwrap() + "/.config/kalc.vars");
    #[cfg(not(unix))]
    let file_path = &format!(
        "C:\\Users\\{}\\AppData\\Roaming\\kalc.vars",
        var("USERNAME").unwrap()
    );
    let mut vars: Vec<Variable> = if options.allow_vars
        && (options.interactive || options.stay_interactive)
    {
        get_vars(options)
    }
    else
    {
        Vec::new()
    };
    let base = options.base;
    if !options.interactive && options.allow_vars && !options.stay_interactive
    {
        get_cli_vars(options, args.concat(), &mut vars)
    }
    {
        if options.allow_vars
        {
            options.base = (10, 10);
            if let Ok(file) = File::open(file_path)
            {
                let lines = BufReader::new(file)
                    .lines()
                    .filter_map(|l| {
                        let l = l.unwrap();
                        if !l.starts_with('#') && !l.is_empty()
                        {
                            Some(l)
                        }
                        else
                        {
                            None
                        }
                    })
                    .collect::<Vec<String>>();
                let mut split;
                let args = args.concat();
                let mut blacklist = if options.interactive || options.stay_interactive
                {
                    Vec::new()
                }
                else
                {
                    vars.iter()
                        .map(|v| v.name.iter().collect::<String>())
                        .collect::<Vec<String>>()
                };
                'upper: for i in lines.clone()
                {
                    split = i.splitn(2, '=');
                    let l = split.next().unwrap().to_string();
                    let left = if l.contains('(')
                    {
                        l.split('(').next().unwrap().to_owned()
                    }
                    else
                    {
                        l.clone()
                    };
                    if options.interactive
                        || options.stay_interactive
                        || (!blacklist.contains(&l)
                            && if l.contains('(')
                            {
                                args.contains(&(left.clone() + "("))
                                    || args.contains(&(left.clone() + "{"))
                                    || args.contains(&(left.clone() + "["))
                                    || args.contains(&(left.clone() + "|"))
                            }
                            else
                            {
                                args.contains(&left)
                            })
                    {
                        if let Some(r) = split.next()
                        {
                            let le = l.chars().collect::<Vec<char>>();
                            if !options.interactive && !options.stay_interactive
                            {
                                get_file_vars(options, &mut vars, lines.clone(), r, &mut blacklist);
                                if blacklist.contains(&l)
                                {
                                    continue;
                                }
                                blacklist.push(l);
                            }
                            for (i, v) in vars.iter().enumerate()
                            {
                                if v.name.split(|c| c == &'(').next()
                                    == le.split(|c| c == &'(').next()
                                    && v.name.contains(&'(') == le.contains(&'(')
                                    && v.name.iter().filter(|c| c == &&',').count()
                                        == le.iter().filter(|c| c == &&',').count()
                                {
                                    if r == "null"
                                    {
                                        if let Err(s) =
                                            add_var(le, r, i, &mut vars, options, true, true, true)
                                        {
                                            println!("{}", s)
                                        }
                                    }
                                    else if let Err(s) =
                                        add_var(le, r, i, &mut vars, options, true, true, false)
                                    {
                                        println!("{}", s)
                                    }
                                    continue 'upper;
                                }
                            }
                            for (i, j) in vars.clone().iter().enumerate()
                            {
                                if j.name.len() <= le.len()
                                {
                                    if let Err(s) =
                                        add_var(le, r, i, &mut vars, options, false, false, false)
                                    {
                                        println!("{}", s)
                                    }
                                    continue 'upper;
                                }
                            }
                            if let Err(s) =
                                add_var(le, r, 0, &mut vars, options, false, false, false)
                            {
                                println!("{}", s)
                            }
                        }
                    }
                }
            }
        }
    }
    options.base = base;
    let (mut file, mut unmod_lines) = if options.interactive || options.stay_interactive
    {
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
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let mut cut: Vec<char> = Vec::new();
    loop
    {
        let mut input = Vec::new();
        let mut graphable = false;
        let mut varcheck = false;
        if !args.is_empty()
        {
            let watch = if options.debug
            {
                Some(Instant::now())
            }
            else
            {
                None
            };
            input = args.remove(0).chars().collect();
            let output;
            let funcvar;
            if input.contains(&'#')
            {
                graphable = true
            }
            else
            {
                (output, funcvar, graphable, varcheck) = match input_var(
                    &input.iter().map(convert).collect::<String>(),
                    vars.clone(),
                    &mut Vec::new(),
                    &mut 0,
                    options,
                    false,
                    false,
                    0,
                    Vec::new(),
                )
                {
                    Ok(f) => f,
                    Err(s) =>
                    {
                        println!("{}: {}", input.iter().collect::<String>(), s);
                        continue;
                    }
                };
                if !graphable && !varcheck
                {
                    match do_math(output, options, funcvar)
                    {
                        Ok(n) => print_answer(n, options, &colors),
                        Err(s) =>
                        {
                            println!("{}: {}", input.iter().collect::<String>(), s);
                            continue;
                        }
                    }
                }
                if !graphable && !varcheck
                {
                    if let Some(time) = watch
                    {
                        println!(" {}", time.elapsed().as_nanos());
                    }
                    else
                    {
                        println!();
                    }
                }
            }
            if args.is_empty() && options.stay_interactive
            {
                options.interactive = true;
                terminal::enable_raw_mode().unwrap();
                print!(
                    "\x1b[G\x1b[K{}{}",
                    prompt(options, &colors),
                    if options.color { "\x1b[0m" } else { "" }
                );
                stdout.flush().unwrap();
            }
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
            let mut long = false;
            let mut frac = 0;
            let mut current = Vec::new();
            let mut lines = unmod_lines.clone().unwrap();
            let mut i = lines.len();
            let mut placement = 0;
            let last = if i == 0
            {
                Vec::new()
            }
            else if lines[i - 1].starts_with('\0')
            {
                lines[i - 1][1..].chars().collect::<Vec<char>>()
            }
            else
            {
                lines[i - 1].chars().collect::<Vec<char>>()
            };
            let mut start = 0;
            let mut end = 0;
            let mut slow = false;
            let mut firstslow = false;
            loop
            {
                let c = read_single_char();
                let watch = Instant::now();
                match c
                {
                    '\n' | '\x14' | '\x09' | '\x06' =>
                    {
                        if c != '\x14' && c != '\x06'
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                        }
                        end = start + get_terminal_dimensions().0
                            - if options.prompt { 3 } else { 1 };
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        if (!options.real_time_output || long || (slow && !firstslow))
                            && (!input.is_empty() && !input.starts_with(&['#']))
                            && c != '\x14'
                            && c != '\x06'
                        {
                            (frac, graphable, _, varcheck) = print_concurrent(
                                &input,
                                &last,
                                vars.clone(),
                                options,
                                colors.clone(),
                                start,
                                end,
                                c == '\n',
                            );
                            slow = watch.elapsed().as_millis() > options.slowcheck;
                        }
                        if !input.is_empty() && !input.starts_with(&['#'])
                        {
                            let num = frac + if !graphable && !varcheck { 1 } else { 0 };
                            if num != 0
                            {
                                print!("\x1b[{}B", num);
                            }
                        }
                        if c == '\x14' || c == '\x06'
                        {
                            if input.is_empty()
                            {
                                print!("\x1b[G\x1b[J");
                            }
                            else
                            {
                                print!("\x1b[G\n\x1b[J");
                            }
                            terminal::disable_raw_mode().unwrap();
                            std::process::exit(0);
                        }
                        print!("\x1b[G\n\x1b[K");
                        if c == '\x09'
                        {
                            graphable = false;
                        }
                        break;
                    }
                    '\x03' =>
                    {
                        //ctrl+backspace
                        if placement != 0
                            && matches!(
                                input[placement - 1],
                                '(' | '{'
                                    | '['
                                    | ')'
                                    | '}'
                                    | ']'
                                    | '+'
                                    | '-'
                                    | '*'
                                    | '/'
                                    | '^'
                                    | '<'
                                    | '='
                                    | '>'
                                    | '|'
                                    | '&'
                                    | '!'
                            )
                        {
                            placement -= 1;
                            input.remove(placement);
                        }
                        else
                        {
                            for (i, c) in input[..placement].iter().rev().enumerate()
                            {
                                if c.is_whitespace() || i + 1 == placement
                                {
                                    input.drain(placement - i - 1..placement);
                                    placement -= i + 1;
                                    break;
                                }
                                if matches!(
                                    c,
                                    '(' | '{'
                                        | '['
                                        | ')'
                                        | '}'
                                        | ']'
                                        | '+'
                                        | '-'
                                        | '*'
                                        | '/'
                                        | '^'
                                        | '<'
                                        | '='
                                        | '>'
                                        | '|'
                                        | '&'
                                        | '!'
                                )
                                {
                                    input.drain(placement - i..placement);
                                    placement -= i;
                                    break;
                                }
                            }
                        }
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        if start >= end
                        {
                            start = end;
                        }
                        if i == lines.len()
                        {
                            current.clone_from(&input);
                        }
                        else
                        {
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        if options.real_time_output && !slow
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                vars.clone(),
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck
                            {
                                firstslow = true;
                                slow = true;
                            }
                        }
                        else if firstslow
                        {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, start, end, options, &colors);
                        }
                        if options.debug
                        {
                            let time = watch.elapsed().as_nanos();
                            print!(
                                " {}\x1b[{}D",
                                time,
                                time.to_string().len() + 1 + end - placement
                            );
                        }
                        else if end - placement != 0
                        {
                            print!("\x1b[{}D", end - placement)
                        }
                        if input.is_empty()
                        {
                            slow = false;
                            clear(&input, start, end, options, &colors);
                        }
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
                        end = start + get_terminal_dimensions().0
                            - if options.prompt { 3 } else { 1 };
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        if i == lines.len()
                        {
                            current.clone_from(&input);
                        }
                        else
                        {
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        if options.real_time_output && !slow
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                vars.clone(),
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck
                            {
                                firstslow = true;
                                slow = true;
                            }
                        }
                        else if firstslow
                        {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, start, end, options, &colors);
                        }
                        if options.debug
                        {
                            let time = watch.elapsed().as_nanos();
                            print!(
                                " {}\x1b[{}D",
                                time,
                                time.to_string().len() + 1 + end - placement
                            );
                        }
                        else if end - placement != 0
                        {
                            print!("\x1b[{}D", end - placement)
                        }
                        if input.is_empty()
                        {
                            slow = false;
                            clear(&input, start, end, options, &colors);
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
                        end = start + get_terminal_dimensions().0
                            - if options.prompt { 3 } else { 1 };
                        if end > input.len()
                        {
                            end = input.len()
                        }
                        if i == lines.len()
                        {
                            current.clone_from(&input);
                        }
                        else
                        {
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        if options.real_time_output && !slow
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                vars.clone(),
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck
                            {
                                firstslow = true;
                                slow = true;
                            }
                        }
                        else if firstslow
                        {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, start, end, options, &colors);
                        }
                        if options.debug
                        {
                            let time = watch.elapsed().as_nanos();
                            print!(
                                " {}\x1b[{}D",
                                time,
                                time.to_string().len() + 1 + end - placement
                            );
                        }
                        else if end - placement != 0
                        {
                            print!("\x1b[{}D", end - placement)
                        }
                        if input.is_empty()
                        {
                            slow = false;
                            clear(&input, start, end, options, &colors);
                        }
                    }
                    '\x11' =>
                    {
                        //end
                        placement = input.len();
                        end = input.len();
                        start = if get_terminal_dimensions().0 - if options.prompt { 3 } else { 1 }
                            > input.len()
                        {
                            0
                        }
                        else
                        {
                            input.len()
                                - (get_terminal_dimensions().0 - if options.prompt { 3 } else { 1 })
                        };
                        clearln(&input, start, end, options, &colors);
                    }
                    '\x18' =>
                    {
                        //ctrl+u
                        cut = input.drain(..placement).collect();
                        end -= placement;
                        placement = 0;
                        if options.real_time_output && !slow
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                vars.clone(),
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck
                            {
                                firstslow = true;
                                slow = true;
                            }
                        }
                        else if firstslow
                        {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, start, end, options, &colors);
                        }
                        if end - placement != 0
                        {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                    '\x19' =>
                    {
                        //ctrl+k
                        cut = input.drain(placement..).collect();
                        end = input.len();
                        if options.real_time_output && !slow
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                vars.clone(),
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck
                            {
                                firstslow = true;
                                slow = true;
                            }
                        }
                        else if firstslow
                        {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, start, end, options, &colors);
                        }
                        if end - placement != 0
                        {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                    '\x17' =>
                    {
                        //ctrl+y
                        let mut cut = cut.clone();
                        end += cut.len();
                        cut.extend(input.drain(placement..));
                        input.extend(cut);
                        if options.real_time_output && !slow
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                vars.clone(),
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck
                            {
                                firstslow = true;
                                slow = true;
                            }
                        }
                        else if firstslow
                        {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, start, end, options, &colors);
                        }
                        if end - placement != 0
                        {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                    '\x16' =>
                    {
                        //ctrl+t
                        if placement + 1 < input.len()
                        {
                            let char = input.remove(placement);
                            input.insert(placement + 1, char);
                            if options.real_time_output && !slow
                            {
                                execute!(stdout, DisableBlinking).unwrap();
                                (frac, graphable, long, varcheck) = print_concurrent(
                                    &input,
                                    &last,
                                    vars.clone(),
                                    options,
                                    colors.clone(),
                                    start,
                                    end,
                                    false,
                                );
                                if watch.elapsed().as_millis() > options.slowcheck
                                {
                                    firstslow = true;
                                    slow = true;
                                }
                            }
                            else if firstslow
                            {
                                firstslow = false;
                                handle_err(
                                    "too slow, will print on enter",
                                    &input,
                                    options,
                                    &colors,
                                    start,
                                    end,
                                )
                            }
                            else
                            {
                                clearln(&input, start, end, options, &colors);
                            }
                            if end - placement != 0
                            {
                                print!("\x1b[{}D", end - placement)
                            }
                        }
                    }
                    '\x15' =>
                    {
                        //ctrl+l
                        if options.real_time_output && !slow
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                vars.clone(),
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck
                            {
                                firstslow = true;
                                slow = true;
                            }
                        }
                        else if firstslow
                        {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, start, end, options, &colors);
                        }
                        if end - placement != 0
                        {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                    '\x10' =>
                    {
                        //home
                        placement = 0;
                        start = 0;
                        end = if get_terminal_dimensions().0 - if options.prompt { 3 } else { 1 }
                            > input.len()
                        {
                            input.len()
                        }
                        else
                        {
                            get_terminal_dimensions().0 - if options.prompt { 3 } else { 1 }
                        };
                        clearln(&input, start, end, options, &colors);
                        if end - placement != 0
                        {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                    '\x1D' | '\x05' =>
                    {
                        //up history
                        i -= if i > 0 { 1 } else { 0 };
                        if lines.is_empty()
                        {
                            continue;
                        }
                        input = lines[i].clone().chars().collect::<Vec<char>>();
                        slow = input.starts_with(&['\0']);
                        if slow
                        {
                            input.remove(0);
                            firstslow = false;
                            placement = input.len();
                            end = input.len();
                            handle_err(
                                "too slow, will print on enter",
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            placement = input.len();
                            end = input.len();
                        }
                        start = if get_terminal_dimensions().0 - if options.prompt { 3 } else { 1 }
                            > input.len()
                        {
                            0
                        }
                        else
                        {
                            input.len()
                                - (get_terminal_dimensions().0 - if options.prompt { 3 } else { 1 })
                        };
                        if options.real_time_output && !slow
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                vars.clone(),
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            slow = watch.elapsed().as_millis() > options.slowcheck;
                            firstslow = slow
                        }
                        else
                        {
                            clearln(&input, start, end, options, &colors);
                        }
                    }
                    '\x1E' | '\x04' =>
                    {
                        //down history
                        i += 1;
                        if i >= lines.len()
                        {
                            i = lines.len();
                            input.clone_from(&current);
                        }
                        else
                        {
                            input = lines[i].clone().chars().collect::<Vec<char>>();
                        }
                        slow = input.starts_with(&['\0']);
                        if slow
                        {
                            input.remove(0);
                            firstslow = false;
                            placement = input.len();
                            end = input.len();
                            handle_err(
                                "too slow, will print on enter",
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            placement = input.len();
                            end = input.len();
                        }
                        start = if get_terminal_dimensions().0 - if options.prompt { 3 } else { 1 }
                            > input.len()
                        {
                            0
                        }
                        else
                        {
                            input.len()
                                - (get_terminal_dimensions().0 - if options.prompt { 3 } else { 1 })
                        };
                        if options.real_time_output && !slow
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                vars.clone(),
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            slow = watch.elapsed().as_millis() > options.slowcheck;
                            firstslow = slow;
                        }
                        else
                        {
                            clearln(&input, start, end, options, &colors);
                        }
                    }
                    '\x1B' =>
                    {
                        //left
                        if placement - start == 0 && placement != 0 && start != 0
                        {
                            start -= 1;
                            placement -= 1;
                            end = start + get_terminal_dimensions().0
                                - if options.prompt { 3 } else { 1 };
                            if end > input.len()
                            {
                                end = input.len()
                            }
                            clearln(&input, start, end, options, &colors);
                            print!("\x1b[{}D", end - placement)
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
                        end = start + get_terminal_dimensions().0
                            - if options.prompt { 3 } else { 1 };
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
                                        hit = false;
                                        placement = i + 1;
                                        break;
                                    }
                                }
                                else
                                {
                                    hit = true;
                                }
                            }
                            if hit
                            {
                                placement = 0;
                            }
                            if placement <= start
                            {
                                end = placement
                                    + (get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 });
                                start = placement;
                                clearln(&input, start, end, options, &colors);
                                if end - placement != 0
                                {
                                    print!("\x1b[{}D", end - placement)
                                }
                            }
                            else if placement == s
                            {
                                placement = 0;
                                print!("\x1b[{}D", s);
                            }
                            else
                            {
                                print!("\x1b[{}D", s - placement);
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
                                        hit = false;
                                        placement += i + 1;
                                        break;
                                    }
                                }
                                else
                                {
                                    hit = true;
                                }
                            }
                            if hit
                            {
                                placement = input.len();
                            }
                            if placement >= end
                            {
                                start = placement
                                    - (get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 });
                                end = placement;
                                clearln(&input, start, end, options, &colors)
                            }
                            else if placement == s
                            {
                                placement = input.len();
                                print!("\x1b[{}C", input.len() - s);
                            }
                            else
                            {
                                print!("\x1b[{}C", placement - s);
                            }
                        }
                    }
                    '\0' =>
                    {}
                    _ =>
                    {
                        input.insert(placement, c);
                        placement += 1;
                        end = start + get_terminal_dimensions().0
                            - if options.prompt { 3 } else { 1 }
                            + 1;
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
                            current.clone_from(&input);
                        }
                        else
                        {
                            lines[i] = input.clone().iter().collect::<String>();
                        }
                        if options.real_time_output && !slow
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                vars.clone(),
                                options,
                                colors.clone(),
                                start,
                                end,
                                false,
                            );
                            if watch.elapsed().as_millis() > options.slowcheck
                            {
                                firstslow = true;
                                slow = true;
                            }
                        }
                        else if firstslow
                        {
                            firstslow = false;
                            handle_err(
                                "too slow, will print on enter",
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, start, end, options, &colors);
                        }
                        if options.debug
                        {
                            let time = watch.elapsed().as_nanos();
                            print!(
                                " {}\x1b[{}D",
                                time,
                                time.to_string().len() + 1 + end - placement
                            );
                        }
                        else if end - placement != 0
                        {
                            print!("\x1b[{}D", end - placement)
                        }
                    }
                }
                stdout.flush().unwrap();
            }
            commands(
                &mut options,
                &colors,
                &mut vars,
                &lines,
                &input,
                &mut stdout,
            );
            if !varcheck
            {
                print!(
                    "{}{}",
                    prompt(options, &colors),
                    if options.color { "\x1b[0m" } else { "" }
                );
            }
            stdout.flush().unwrap();
            execute!(stdout, EnableBlinking).unwrap();
            if input.is_empty()
            {
                continue;
            }
            write(
                input
                    .iter()
                    .collect::<String>()
                    .replace('_', &format!("({})", last.iter().collect::<String>())),
                file.as_mut().unwrap(),
                unmod_lines.as_mut().unwrap(),
                slow,
                last.iter().collect::<String>(),
            );
        }
        if varcheck
        {
            if let Err(s) = set_commands_or_vars(&mut colors, &mut options, &mut vars, &input)
            {
                if !s.is_empty()
                {
                    print!("\x1b[G\x1b[K{}\x1b[G\n{}", s, prompt(options, &colors));
                }
                else
                {
                    print!(
                        "{}{}",
                        prompt(options, &colors),
                        if options.color { "\x1b[0m" } else { "" }
                    );
                }
            }
            else
            {
                print!(
                    "{}{}",
                    prompt(options, &colors),
                    if options.color { "\x1b[0m" } else { "" }
                );
            }
            stdout.flush().unwrap()
        }
        else if options.graph && graphable
        {
            let inputs: Vec<String> = input
                .iter()
                .collect::<String>()
                .split('#')
                .map(String::from)
                .collect();
            if inputs.len() < 7
            {
                let watch = if options.debug
                {
                    Some(Instant::now())
                }
                else
                {
                    None
                };
                if options.graph_cli
                {
                    if options.interactive
                    {
                        terminal::disable_raw_mode().unwrap();
                    }
                    graph(inputs, vars.clone(), options, watch, colors.clone(), true)
                        .join()
                        .unwrap();
                    if options.interactive
                    {
                        terminal::enable_raw_mode().unwrap();
                    }
                }
                else
                {
                    handles.push(graph(
                        inputs,
                        vars.clone(),
                        options,
                        watch,
                        colors.clone(),
                        false,
                    ));
                }
            }
        }
    }
}
