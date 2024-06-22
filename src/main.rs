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
//lib, gui
use crate::{
    complex::NumStr,
    functions::{functions_with_args, options_list, units_list},
    graph::graph,
    help::help_for,
    load_vars::{add_var, get_cli_vars, get_file_vars, get_vars, set_commands_or_vars},
    math::do_math,
    misc::{
        clear, clearln, convert, get_terminal_dimensions, handle_err, insert_last, prompt,
        read_single_char, to_output, write,
    },
    options::{arg_opts, commands, equal_to, file_opts, silent_commands},
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
    env::args,
    fs::{File, OpenOptions},
    io::{stdin, stdout, BufRead, BufReader, IsTerminal, Stdout, Write},
    thread::JoinHandle,
    time::Instant,
};
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
    usd: f64,
    unit: f64,
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
    graphtofile: String,
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
            graphtofile: String::new(),
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
pub enum Auto
{
    True,
    False,
    Auto,
}
#[derive(Default, Copy, Clone, PartialEq)]
pub struct HowGraphing
{
    graph: bool,
    x: bool,
    y: bool,
}
#[derive(Copy, Clone, PartialEq)]
pub struct Fractions
{
    num: bool,
    vec: bool,
    mat: bool,
}
#[derive(Copy, Clone, PartialEq)]
pub enum Notation
{
    Normal,
    Scientific,
    LargeEngineering,
    SmallEngineering,
}
#[derive(Copy, Clone, PartialEq)]
pub enum GraphType
{
    Normal,
    Flat,
    Depth,
    None,
}
#[derive(Clone, Copy)]
pub struct Options
{
    notation: Notation,
    angle: AngleType,
    graphtype: GraphType,
    base: (i32, i32),
    ticks: (f64, f64, f64),
    onaxis: bool,
    polar: bool,
    frac: Fractions,
    real_time_output: bool,
    decimal_places: usize,
    color: Auto,
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
    lines: Auto,
    multi: bool,
    tabbed: bool,
    allow_vars: bool,
    debug: bool,
    slowcheck: u128,
    interactive: bool,
    surface: bool,
    scale_graph: bool,
    stay_interactive: bool,
    graph_cli: bool,
    units: bool,
    si_units: bool,
    window_size: (usize, usize),
    keep_zeros: bool,
}
impl Default for Options
{
    fn default() -> Self
    {
        Self {
            notation: Normal,
            angle: Radians,
            graphtype: GraphType::Normal,
            base: (10, 10),
            ticks: (16.0, 16.0, 16.0),
            onaxis: true,
            polar: false,
            frac: Fractions {
                num: true,
                vec: true,
                mat: false,
            },
            real_time_output: true,
            decimal_places: 12,
            color: Auto::Auto,
            prompt: true,
            comma: false,
            prec: 512,
            graph_prec: 128,
            xr: (-8.0, 8.0),
            yr: (-8.0, 8.0),
            zr: (-8.0, 8.0),
            vxr: (0.0, 0.0),
            vyr: (0.0, 0.0),
            vzr: (0.0, 0.0),
            samples_2d: 8192,
            samples_3d: (256, 256),
            point_style: '.',
            lines: Auto::Auto,
            multi: true,
            tabbed: false,
            allow_vars: true,
            debug: false,
            slowcheck: 256,
            interactive: true,
            surface: false,
            scale_graph: false,
            stay_interactive: false,
            graph_cli: false,
            units: true,
            si_units: false,
            window_size: (0, 0),
            keep_zeros: false,
        }
    }
}
fn main()
{
    let mut colors = Colors::default();
    let mut options = Options::default();
    let mut args = args().collect::<Vec<String>>();
    let mut default = false;
    let dir = dirs::config_dir().unwrap().to_str().unwrap().to_owned() + "/kalc";
    std::fs::create_dir_all(dir.clone()).unwrap();
    {
        let file_path = dir.clone() + "/kalc.config";
        if let Err(s) = file_opts(&mut options, &mut colors, &file_path)
        {
            println!("{}", s);
            std::process::exit(1);
        }
        match arg_opts(&mut options, &mut colors, &mut args)
        {
            Ok(s) =>
            {
                if s
                {
                    default = true
                }
            }
            Err(s) =>
            {
                println!("{}", s);
                std::process::exit(1);
            }
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
        if options.color == Auto::Auto
        {
            options.color = Auto::True;
        }
        terminal::enable_raw_mode().unwrap();
        print!(
            "\x1b[G\x1b[K{}{}",
            prompt(options, &colors),
            if options.color == Auto::True
            {
                "\x1b[0m"
            }
            else
            {
                ""
            }
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
    let file_path = dir.clone() + "/kalc.vars";
    let mut vars: Vec<Variable> = if options.allow_vars
        && (options.interactive || options.stay_interactive)
    {
        get_vars(options)
    }
    else
    {
        Vec::new()
    };
    let mut err = false;
    let base = options.base;
    if !options.interactive && options.allow_vars && !options.stay_interactive
    {
        get_cli_vars(options, args.join(" "), &mut vars)
    }
    {
        if options.allow_vars && !default
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
                let args = args.join(" ");
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
                        || (!blacklist.contains(&l) && {
                            let mut b = false;
                            let mut word = String::new();
                            for c in args.chars()
                            {
                                if c.is_alphanumeric() || matches!(c, '\'' | '`' | '_')
                                {
                                    word.push(c)
                                }
                                else
                                {
                                    if l.contains('(')
                                    {
                                        b = word.trim_end_matches('\'').trim_end_matches('`')
                                            == left
                                            && matches!(c, '(' | '{' | '[' | '|');
                                    }
                                    else
                                    {
                                        b = word == left;
                                    }
                                    if b
                                    {
                                        break;
                                    }
                                    word.clear()
                                }
                            }
                            b
                        })
                    {
                        if let Some(r) = split.next()
                        {
                            let le = l.chars().collect::<Vec<char>>();
                            if !options.interactive && !options.stay_interactive
                            {
                                blacklist.push(l);
                                get_file_vars(options, &mut vars, lines.clone(), r, &mut blacklist);
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
                                            err = true;
                                            println!("\x1b[G\x1b[K{}", s)
                                        }
                                    }
                                    else if let Err(s) =
                                        add_var(le, r, i, &mut vars, options, true, true, false)
                                    {
                                        err = true;
                                        println!("\x1b[G\x1b[K{}", s)
                                    }
                                    continue 'upper;
                                }
                            }
                            for (i, j) in vars.iter().enumerate()
                            {
                                if j.name.len() <= le.len()
                                {
                                    if let Err(s) =
                                        add_var(le, r, i, &mut vars, options, false, false, false)
                                    {
                                        err = true;
                                        println!("\x1b[G\x1b[K{}", s)
                                    }
                                    continue 'upper;
                                }
                            }
                            if let Err(s) =
                                add_var(le, r, 0, &mut vars, options, false, false, false)
                            {
                                err = true;
                                println!("\x1b[G\x1b[K{}", s)
                            }
                        }
                    }
                }
            }
        }
    }
    if options.interactive && err
    {
        print!(
            "\x1b[G\x1b[K{}{}",
            prompt(options, &colors),
            if options.color == Auto::True
            {
                "\x1b[0m"
            }
            else
            {
                ""
            }
        );
        stdout.flush().unwrap();
    }
    options.base = base;
    let (mut file, mut unmod_lines) = if options.interactive || options.stay_interactive
    {
        if options.color == Auto::Auto
        {
            options.color = Auto::True;
        }
        let file_path = &(dir.clone() + "/kalc.history");
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
        if options.color == Auto::Auto
        {
            options.color = Auto::False;
        }
        (None, None)
    };
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let mut cut: Vec<char> = Vec::new();
    'main: loop
    {
        let mut input = Vec::new();
        let mut graphable = HowGraphing::default();
        let mut varcheck = false;
        let mut last = Vec::new();
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
            {
                let mut options = options;
                let mut unparsed = input.clone();
                {
                    let split = input.split(|c| c == &';');
                    let count = split.clone().count();
                    if count != 1
                    {
                        unparsed = split.clone().last().unwrap().to_vec();
                        for (i, s) in split.enumerate()
                        {
                            if i == count - 1
                            {
                                break;
                            }
                            silent_commands(
                                &mut options,
                                &s.iter()
                                    .copied()
                                    .filter(|&c| !c.is_whitespace())
                                    .collect::<Vec<char>>(),
                            );
                            if s.contains(&'=')
                            {
                                if let Err(s) =
                                    set_commands_or_vars(&mut colors, &mut options, &mut vars, s)
                                {
                                    println!("{}", s);
                                    continue 'main;
                                }
                            }
                        }
                    }
                    let tempinput = unparsed.iter().collect::<String>();
                    if tempinput.starts_with("help ")
                    {
                        println!("{}", help_for(tempinput.splitn(2, ' ').last().unwrap()));
                        continue;
                    }
                    else if tempinput.ends_with('=')
                    {
                        println!(
                            "{}",
                            equal_to(
                                options,
                                &colors,
                                &vars,
                                &tempinput[..tempinput.len().saturating_sub(1)],
                                "",
                            )
                        );
                        continue;
                    }
                }
                (output, funcvar, graphable, varcheck, _) = match input_var(
                    &unparsed.iter().map(convert).collect::<String>(),
                    &vars,
                    &mut Vec::new(),
                    &mut 0,
                    options,
                    false,
                    0,
                    Vec::new(),
                    false,
                    &mut Vec::new(),
                    None,
                )
                {
                    Ok(f) => f,
                    Err(s) =>
                    {
                        println!("{}: {}", input.iter().collect::<String>(), s);
                        continue;
                    }
                };
                if !graphable.graph && !varcheck
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
        }
        else
        {
            if !options.interactive
            {
                if options.stay_interactive
                {
                    setup_for_interactive(&colors, &mut options, &mut stdout)
                }
                else
                {
                    for handle in handles
                    {
                        handle.join().unwrap();
                    }
                    break;
                }
            }
            let mut long = false;
            let mut frac = 0;
            let mut current = Vec::new();
            let mut lines = unmod_lines.clone().unwrap();
            let mut i = lines.len();
            let mut placement = 0;
            last = if i == 0
            {
                Vec::new()
            }
            else if lines[i - 1].ends_with('\t')
            {
                lines[i - 1][..lines[i - 1].len() - 1]
                    .chars()
                    .collect::<Vec<char>>()
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
                                &vars,
                                options,
                                colors.clone(),
                                start,
                                end,
                                c == '\n',
                            );
                            slow = watch.elapsed().as_millis() > options.slowcheck;
                        }
                        if c == '\x09'
                        {
                            clear(&input, &vars, start, end, options, &colors);
                        }
                        else
                        {
                            if !input.is_empty() && !input.starts_with(&['#']) && frac != 0
                            {
                                print!("\x1b[{}B", frac);
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
                        }
                        print!("\x1b[G\n\x1b[K");
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
                                &vars,
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
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, &vars, start, end, options, &colors);
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
                            clear(&input, &vars, start, end, options, &colors);
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
                                &vars,
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
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, &vars, start, end, options, &colors);
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
                            clear(&input, &vars, start, end, options, &colors);
                        }
                    }
                    '\x7F' =>
                    {
                        //delete
                        if placement == input.len()
                        {
                            continue;
                        }
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
                                &vars,
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
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, &vars, start, end, options, &colors);
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
                            clear(&input, &vars, start, end, options, &colors);
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
                        clearln(&input, &vars, start, end, options, &colors);
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
                                &vars,
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
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, &vars, start, end, options, &colors);
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
                                &vars,
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
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, &vars, start, end, options, &colors);
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
                                &vars,
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
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, &vars, start, end, options, &colors);
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
                                    &vars,
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
                                    &vars,
                                    &input,
                                    options,
                                    &colors,
                                    start,
                                    end,
                                )
                            }
                            else
                            {
                                clearln(&input, &vars, start, end, options, &colors);
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
                                &vars,
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
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, &vars, start, end, options, &colors);
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
                        clearln(&input, &vars, start, end, options, &colors);
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
                        slow = input.ends_with(&['\t']);

                        if slow
                        {
                            input.pop();
                            firstslow = false;
                            placement = input.len();
                            end = input.len();
                            start = if get_terminal_dimensions().0
                                - if options.prompt { 3 } else { 1 }
                                > input.len()
                            {
                                0
                            }
                            else
                            {
                                input.len()
                                    - (get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 })
                            };
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
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
                            start = if get_terminal_dimensions().0
                                - if options.prompt { 3 } else { 1 }
                                > input.len()
                            {
                                0
                            }
                            else
                            {
                                input.len()
                                    - (get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 })
                            };
                        }
                        if options.real_time_output && !slow
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
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
                            clearln(&input, &vars, start, end, options, &colors);
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
                        slow = input.ends_with(&['\t']);
                        if slow
                        {
                            input.pop();
                            firstslow = false;
                            placement = input.len();
                            end = input.len();
                            start = if get_terminal_dimensions().0
                                - if options.prompt { 3 } else { 1 }
                                > input.len()
                            {
                                0
                            }
                            else
                            {
                                input.len()
                                    - (get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 })
                            };
                            handle_err(
                                "too slow, will print on enter",
                                &vars,
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
                            start = if get_terminal_dimensions().0
                                - if options.prompt { 3 } else { 1 }
                                > input.len()
                            {
                                0
                            }
                            else
                            {
                                input.len()
                                    - (get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 })
                            };
                        }
                        if options.real_time_output && !slow
                        {
                            execute!(stdout, DisableBlinking).unwrap();
                            (frac, graphable, long, varcheck) = print_concurrent(
                                &input,
                                &last,
                                &vars,
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
                            clearln(&input, &vars, start, end, options, &colors);
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
                            clearln(&input, &vars, start, end, options, &colors);
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
                            clearln(&input, &vars, start, end, options, &colors)
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
                                if end > input.len()
                                {
                                    end = input.len()
                                }
                                start = placement;
                                clearln(&input, &vars, start, end, options, &colors);
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
                                start = placement.saturating_sub(
                                    get_terminal_dimensions().0
                                        - if options.prompt { 3 } else { 1 },
                                );
                                end = placement;
                                clearln(&input, &vars, start, end, options, &colors)
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
                    '\x1F' =>
                    {
                        //tab completion
                        let mut word = String::new();
                        for (i, c) in input[..placement].iter().rev().enumerate()
                        {
                            if c.is_alphabetic()
                                || matches!(*c, '°' | '\'' | '`' | '_' | '∫' | '$' | '¢')
                            {
                                word.insert(0, *c)
                            }
                            else if !(*c == '(' && i == 0)
                            {
                                break;
                            }
                        }
                        if !word.is_empty()
                        {
                            let mut bank = Vec::new();
                            for v in &vars
                            {
                                let s = v.name.iter().collect::<String>();
                                if s.starts_with(&word)
                                {
                                    bank.push(s)
                                }
                            }
                            let mut bank_temp = Vec::new();
                            for f in functions_with_args()
                            {
                                if f.starts_with(&word)
                                    && !bank.iter().any(|b| {
                                        b.contains('(')
                                            && b.split('(').next().unwrap()
                                                == f.split('(').next().unwrap()
                                    })
                                {
                                    bank_temp.push(f.to_string())
                                }
                            }
                            bank.extend(bank_temp);
                            let mut bank_temp = Vec::new();
                            for f in options_list()
                            {
                                if f.starts_with(&word)
                                    && !bank.iter().any(|b| {
                                        b.contains('(')
                                            && b.split('(').next().unwrap()
                                                == f.split('(').next().unwrap()
                                    })
                                {
                                    bank_temp.push(f.to_string())
                                }
                            }
                            bank.extend(bank_temp);
                            if options.units
                            {
                                let mut bank_temp = Vec::new();
                                for f in units_list()
                                {
                                    if f.starts_with(&word)
                                        && !bank.iter().any(|b| {
                                            b.contains('(')
                                                && b.split('(').next().unwrap()
                                                    == f.split('(').next().unwrap()
                                        })
                                    {
                                        bank_temp.push(f.to_string())
                                    }
                                }
                                bank.extend(bank_temp);
                            }
                            bank.sort();
                            let mut var = false;
                            if bank.len() == 1
                            {
                                let mut w = bank[0].to_string();
                                if w.contains('(')
                                {
                                    w = w.split('(').next().unwrap().to_string();
                                    if (placement == input.len() || input[placement] != '(')
                                        && input[placement - 1] != '('
                                    {
                                        w.push('(')
                                    }
                                }
                                else
                                {
                                    var = true
                                }
                                let w = w.chars().collect::<Vec<char>>();
                                input.splice(
                                    placement..placement,
                                    w[word.chars().count()..].iter().collect::<String>().chars(),
                                );
                                placement += w.len() - word.chars().count();
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
                                if options.real_time_output && !slow && var
                                {
                                    execute!(stdout, DisableBlinking).unwrap();
                                    (frac, graphable, long, varcheck) = print_concurrent(
                                        &input,
                                        &last,
                                        &vars,
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
                                else if firstslow && var
                                {
                                    firstslow = false;
                                    handle_err(
                                        "too slow, will print on enter",
                                        &vars,
                                        &input,
                                        options,
                                        &colors,
                                        start,
                                        end,
                                    )
                                }
                                else
                                {
                                    clear(&input, &vars, start, end, options, &colors);
                                }
                                if end - placement != 0
                                {
                                    print!("\x1b[{}D", end - placement)
                                }
                            }
                            else if !bank.is_empty()
                            {
                                let mut k = 0;
                                let mut char = '\0';
                                'upper: for n in
                                    0..bank.iter().fold(usize::MAX, |min, str| min.min(str.len()))
                                {
                                    for b in &bank
                                    {
                                        let c = b.chars().nth(n).unwrap();
                                        if char == '\0'
                                        {
                                            char = c
                                        }
                                        else if c != char
                                        {
                                            break 'upper;
                                        }
                                    }
                                    k += 1;
                                    char = '\0'
                                }
                                input.splice(
                                    placement..placement,
                                    bank[0][word.chars().count()..k].chars(),
                                );
                                placement += k - word.chars().count();
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
                                clear(&input, &vars, start, end, options, &colors);
                                if end - placement != 0
                                {
                                    print!("\x1b[{}D", end - placement)
                                }
                            }
                            if !var && !bank.is_empty()
                            {
                                let width = get_terminal_dimensions().0;
                                let mut n = 1;
                                let tab = bank.iter().fold(0, |max, str| max.max(str.len())) + 3;
                                let mut len = 0;
                                print!("\x1b[G\n\x1b[K");
                                for b in bank
                                {
                                    if len + tab > width
                                    {
                                        len = 0;
                                        n += 1;
                                        print!("\x1b[G\n\x1b[K")
                                    }
                                    len += tab;
                                    print!(
                                        "{}{}",
                                        to_output(
                                            &b.chars().collect::<Vec<char>>(),
                                            &vars,
                                            options.color == Auto::True,
                                            &colors
                                        ),
                                        " ".repeat(tab - b.chars().count())
                                    )
                                }
                                print!(
                                    "\x1b[G\x1b[{}A\x1b[{}C",
                                    n,
                                    placement + if options.prompt { 2 } else { 0 }
                                )
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
                                &vars,
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
                                &vars,
                                &input,
                                options,
                                &colors,
                                start,
                                end,
                            )
                        }
                        else
                        {
                            clearln(&input, &vars, start, end, options, &colors);
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
                    if options.color == Auto::True
                    {
                        "\x1b[0m"
                    }
                    else
                    {
                        ""
                    }
                );
            }
            stdout.flush().unwrap();
            execute!(stdout, EnableBlinking).unwrap();
            if input.is_empty()
            {
                continue;
            }
            write(
                insert_last(&input, last.iter().collect::<String>().as_str()),
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
                    print!(
                        "\x1b[G\x1b[A\x1b[K{}\x1b[G\n{}",
                        s,
                        prompt(options, &colors)
                    );
                }
                else
                {
                    print!(
                        "{}{}",
                        prompt(options, &colors),
                        if options.color == Auto::True
                        {
                            "\x1b[0m"
                        }
                        else
                        {
                            ""
                        }
                    );
                }
            }
            else
            {
                print!(
                    "{}{}",
                    prompt(options, &colors),
                    if options.color == Auto::True
                    {
                        "\x1b[0m"
                    }
                    else
                    {
                        ""
                    }
                );
            }
            stdout.flush().unwrap()
        }
        else if graphable.graph
        {
            let inputs: Vec<String> = insert_last(&input, &last.iter().collect::<String>())
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
fn setup_for_interactive(colors: &Colors, options: &mut Options, stdout: &mut Stdout)
{
    options.interactive = true;
    terminal::enable_raw_mode().unwrap();
    print!(
        "\x1b[G\x1b[K{}{}",
        prompt(*options, colors),
        if options.color == Auto::True
        {
            &colors.text
        }
        else
        {
            ""
        }
    );
    stdout.flush().unwrap();
}
