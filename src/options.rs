use crate::{
    complex::{
        NumStr,
        NumStr::{Matrix, Num, Str, Vector},
    },
    help::help,
    load_vars::get_vars,
    math::do_math,
    misc::{prompt, to_output},
    parse::input_var,
    print::get_output,
    AngleType::{Degrees, Gradians, Radians},
    Colors, Options, Variable,
};
use crossterm::{
    execute, terminal,
    terminal::{Clear, ClearType},
};
use rug::Complex;
use std::{
    fs::File,
    io,
    io::{BufRead, BufReader, Stdout, Write},
};
pub fn arg_opts(
    options: &mut Options,
    colors: &mut Colors,
    args: &mut Vec<String>,
) -> Result<(), &'static str>
{
    args.remove(0);
    let mut i = 0;
    while i < args.len()
    {
        if args[i].starts_with("--") && (args[i].contains('=') || args[i].contains(','))
        {
            let l = args[i].clone();
            let mut split = l.split(|c| c == '=' || c == ',');
            args[i] = split.next().unwrap().to_string();
            args.insert(i + 1, split.next().unwrap().to_string());
            if split.clone().count() > 0
            {
                args.insert(i + 2, split.next().unwrap().to_string());
            }
        }
        match args[i].as_str()
        {
            "--" =>
            {
                args.remove(i);
                break;
            }
            "--debug" => options.debug = !options.debug,
            "--depth" => options.depth = !options.depth,
            "--flat" => options.flat = !options.flat,
            "--tau" => options.tau = !options.tau,
            "--small_e" => options.small_e = !options.small_e,
            "--rad" => options.deg = Radians,
            "--deg" => options.deg = Degrees,
            "--grad" => options.deg = Gradians,
            "--prompt" => options.prompt = !options.prompt,
            "--color" => options.color = !options.color,
            "--line" | "--lines" => options.lines = !options.lines,
            "--rt" => options.real_time_output = !options.real_time_output,
            "--polar" => options.polar = !options.polar,
            "--frac" => options.frac = !options.frac,
            "--multi" => options.multi = !options.multi,
            "--tabbed" => options.tabbed = !options.tabbed,
            "--prec" | "--precision" =>
            {
                if args.len() > 1
                {
                    options.prec = match args[i + 1].parse::<u32>()
                    {
                        Ok(x) if x != 0 => (x, x),
                        _ =>
                        {
                            let a = match input_var(
                                &args[i + 1],
                                Vec::new(),
                                &mut Vec::new(),
                                &mut 0,
                                *options,
                                false,
                                &mut (false, 0, 0),
                                false,
                                0,
                                Vec::new(),
                            )
                            {
                                Ok(n) => n.0,
                                _ => return Err("invalid prec"),
                            };
                            match do_math(a, *options, Vec::new())
                            {
                                Ok(Num(n)) => (n.real().to_f64() as u32, n.real().to_f64() as u32),
                                _ => return Err("invalid prec"),
                            }
                        }
                    };
                    args.remove(i);
                }
            }
            "--graphprec" | "--graphprecision" =>
            {
                if args.len() > 1
                {
                    options.graph_prec = match args[i + 1].parse::<u32>()
                    {
                        Ok(x) if x != 0 => (x, x),
                        _ =>
                        {
                            let a = match input_var(
                                &args[i + 1],
                                Vec::new(),
                                &mut Vec::new(),
                                &mut 0,
                                *options,
                                false,
                                &mut (false, 0, 0),
                                false,
                                0,
                                Vec::new(),
                            )
                            {
                                Ok(n) => n.0,
                                _ => return Err("invalid graphprec"),
                            };
                            match do_math(a, *options, Vec::new())
                            {
                                Ok(Num(n)) => (n.real().to_f64() as u32, n.real().to_f64() as u32),
                                _ => return Err("invalid graphprec"),
                            }
                        }
                    };
                    args.remove(i);
                }
            }
            "--decimal" | "--deci" | "--decimals" =>
            {
                if args.len() > 1
                {
                    if args[i + 1] == "-1"
                    {
                        options.decimal_places = usize::MAX - 1;
                    }
                    else if args[i + 1] == "-2"
                    {
                        options.decimal_places = usize::MAX;
                    }
                    else
                    {
                        options.decimal_places = match args[i + 1].parse::<usize>()
                        {
                            Ok(x) if x != 0 => x,
                            _ =>
                            {
                                let a = match input_var(
                                    &args[i + 1],
                                    Vec::new(),
                                    &mut Vec::new(),
                                    &mut 0,
                                    *options,
                                    false,
                                    &mut (false, 0, 0),
                                    false,
                                    0,
                                    Vec::new(),
                                )
                                {
                                    Ok(n) => n.0,
                                    _ => return Err("invalid deci"),
                                };
                                match do_math(a, *options, Vec::new())
                                {
                                    Ok(Num(n)) => n.real().to_f64() as usize,
                                    _ => return Err("invalid deci"),
                                }
                            }
                        };
                    }
                    args.remove(i);
                }
            }
            "--frac_iter" =>
            {
                if args.len() > 1
                {
                    options.frac_iter = args[i + 1].parse::<usize>().expect("invalid frac_iter");
                    args.remove(i);
                }
            }
            "--textc" =>
            {
                if args.len() > 1
                {
                    colors.text = "\x1b[".to_owned() + &args[i + 1];
                    args.remove(i);
                }
            }
            "--promptc" =>
            {
                if args.len() > 1
                {
                    colors.prompt = "\x1b[".to_owned() + &args[i + 1];
                    args.remove(i);
                }
            }
            "--imagc" =>
            {
                if args.len() > 1
                {
                    colors.imag = "\x1b[".to_owned() + &args[i + 1];
                    args.remove(i);
                }
            }
            "--scic" =>
            {
                if args.len() > 1
                {
                    colors.sci = "\x1b[".to_owned() + &args[i + 1];
                    args.remove(i);
                }
            }
            "--bracketc" =>
            {
                if args.len() > 1
                {
                    args.remove(i);
                    colors.brackets.clear();
                    while i < args.len() && args[i].contains('m')
                    {
                        colors.brackets.push("\x1b[".to_owned() + &args[i]);
                        args.remove(i);
                    }
                    continue;
                }
            }
            "--2d" =>
            {
                if args.len() > 1
                {
                    options.samples_2d = args[i + 1].parse::<usize>().expect("invalid 2d");
                    args.remove(i);
                }
            }
            "--3d" =>
            {
                if args.len() > 2
                {
                    options.samples_3d.0 = args[i + 1].parse::<usize>().expect("invalid 3d");
                    options.samples_3d.1 = args[i + 2].parse::<usize>().expect("invalid 3d");
                    args.remove(i);
                    args.remove(i);
                }
            }
            "--yr" =>
            {
                if args.len() > 2
                {
                    options.yr.0 = args[i + 1].parse::<f64>().expect("invalid yr");
                    options.yr.1 = args[i + 2].parse::<f64>().expect("invalid yr");
                    args.remove(i);
                    args.remove(i);
                }
            }
            "--range" =>
            {
                if args.len() > 1
                {
                    let n = args[i + 1].parse::<f64>().expect("invalid range");
                    options.xr.0 = -n;
                    options.xr.1 = n;
                    options.yr.0 = -n;
                    options.yr.1 = n;
                    options.zr.0 = -n;
                    options.zr.1 = n;
                    args.remove(i);
                }
            }
            "--xr" =>
            {
                if args.len() > 2
                {
                    options.xr.0 = args[i + 1].parse::<f64>().expect("invalid xr");
                    options.xr.1 = args[i + 2].parse::<f64>().expect("invalid xr");
                    args.remove(i);
                    args.remove(i);
                }
            }
            "--zr" =>
            {
                if args.len() > 2
                {
                    options.zr.0 = args[i + 1].parse::<f64>().expect("invalid zr");
                    options.zr.1 = args[i + 2].parse::<f64>().expect("invalid zr");
                    args.remove(i);
                    args.remove(i);
                }
            }
            "--base" =>
            {
                if args.len() > 1
                {
                    options.base = match args[i + 1].parse::<usize>()
                    {
                        Ok(x) if (2..=36).contains(&x) => x,
                        _ => return Err("invalid base"),
                    };
                    args.remove(i);
                }
            }
            "--ticks" =>
            {
                if args.len() > 1
                {
                    options.ticks = match args[i + 1].parse::<f64>()
                    {
                        Ok(x) => x,
                        _ => return Err("invalid ticks"),
                    };
                    args.remove(i);
                }
            }
            "--comma" => options.comma = !options.comma,
            "--graph" => options.graph = !options.graph,
            "--sci" | "--scientific" => options.sci = !options.sci,
            "--vars" => options.allow_vars = !options.allow_vars,
            "--point" =>
            {
                options.point_style = match args[i + 1].chars().next()
                {
                    Some(x)
                        if matches!(
                            x,
                            '.' | '+'
                                | 'x'
                                | '*'
                                | 's'
                                | 'S'
                                | 'o'
                                | 'O'
                                | 't'
                                | 'T'
                                | 'd'
                                | 'D'
                                | 'R'
                        ) =>
                    {
                        x
                    }
                    _ => return Err("invalid point"),
                };
                args.remove(i);
            }
            "--help" | "-h" =>
            {
                help();
                std::process::exit(0);
            }
            "--version" | "-v" =>
            {
                println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            "--default" | "--defaults" | "--def" =>
            {
                *options = Options::default();
            }
            _ =>
            {
                i += 1;
                continue;
            }
        }
        args.remove(i);
    }
    Ok(())
}
pub fn file_opts(
    options: &mut Options,
    colors: &mut Colors,
    file_path: &String,
) -> Result<(), &'static str>
{
    if File::open(file_path).is_ok()
    {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let mut split;
        for line in reader.lines().map(|l| l.unwrap())
        {
            if line.starts_with('#')
            {
                continue;
            }
            split = line.split('=');
            match split.next().unwrap()
            {
                "var_multiply" =>
                {
                    options.var_multiply = split
                        .next()
                        .unwrap()
                        .parse::<bool>()
                        .expect("invalid var_multiply")
                }
                "slowcheck" =>
                {
                    options.slowcheck = split
                        .next()
                        .unwrap()
                        .parse::<u128>()
                        .expect("invalid slowcheck")
                }
                "frac_iter" =>
                {
                    options.frac_iter = split
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .expect("invalid frac_iter")
                }
                "re1col" =>
                {
                    colors.re1col = split
                        .next()
                        .unwrap()
                        .parse::<String>()
                        .expect("invalid col")
                }
                "im1col" =>
                {
                    colors.im1col = split
                        .next()
                        .unwrap()
                        .parse::<String>()
                        .expect("invalid col")
                }
                "re2col" =>
                {
                    colors.re2col = split
                        .next()
                        .unwrap()
                        .parse::<String>()
                        .expect("invalid col")
                }
                "im2col" =>
                {
                    colors.im2col = split
                        .next()
                        .unwrap()
                        .parse::<String>()
                        .expect("invalid col")
                }
                "re3col" =>
                {
                    colors.re3col = split
                        .next()
                        .unwrap()
                        .parse::<String>()
                        .expect("invalid col")
                }
                "im3col" =>
                {
                    colors.im3col = split
                        .next()
                        .unwrap()
                        .parse::<String>()
                        .expect("invalid col")
                }
                "re4col" =>
                {
                    colors.re4col = split
                        .next()
                        .unwrap()
                        .parse::<String>()
                        .expect("invalid col")
                }
                "im4col" =>
                {
                    colors.im4col = split
                        .next()
                        .unwrap()
                        .parse::<String>()
                        .expect("invalid col")
                }
                "re5col" =>
                {
                    colors.re5col = split
                        .next()
                        .unwrap()
                        .parse::<String>()
                        .expect("invalid col")
                }
                "im5col" =>
                {
                    colors.im5col = split
                        .next()
                        .unwrap()
                        .parse::<String>()
                        .expect("invalid col")
                }
                "re6col" =>
                {
                    colors.re6col = split
                        .next()
                        .unwrap()
                        .parse::<String>()
                        .expect("invalid col")
                }
                "im6col" =>
                {
                    colors.im6col = split
                        .next()
                        .unwrap()
                        .parse::<String>()
                        .expect("invalid col")
                }
                "textc" =>
                {
                    colors.text = "\x1b[".to_owned()
                        + &split
                            .next()
                            .unwrap()
                            .parse::<String>()
                            .expect("invalid col")
                }
                "promptc" =>
                {
                    colors.prompt = "\x1b[".to_owned()
                        + &split
                            .next()
                            .unwrap()
                            .parse::<String>()
                            .expect("invalid col")
                }
                "imagc" =>
                {
                    colors.imag = "\x1b[".to_owned()
                        + &split
                            .next()
                            .unwrap()
                            .parse::<String>()
                            .expect("invalid col")
                }
                "scic" =>
                {
                    colors.sci = "\x1b[".to_owned()
                        + &split
                            .next()
                            .unwrap()
                            .parse::<String>()
                            .expect("invalid col")
                }
                "bracketc" =>
                {
                    colors.brackets.clear();
                    for i in split.next().unwrap().split(',')
                    {
                        colors.brackets.push("\x1b[".to_owned() + i)
                    }
                }
                "2d" =>
                {
                    options.samples_2d = split.next().unwrap().parse::<usize>().expect("invalid 2d")
                }
                "3d" =>
                {
                    let mut den = split.next().unwrap().split(',');
                    if den.clone().count() != 2
                    {
                        return Err("invalid 3d");
                    }
                    options.samples_3d.0 =
                        den.next().unwrap().parse::<usize>().expect("invalid 3d");
                    options.samples_3d.1 = den.next().unwrap().parse::<usize>().expect("invalid 3d")
                }
                "range" =>
                {
                    let n = split.next().unwrap().parse::<f64>().expect("invalid range");
                    options.xr.0 = -n;
                    options.xr.1 = n;
                    options.yr.0 = -n;
                    options.yr.1 = n;
                    options.zr.0 = -n;
                    options.zr.1 = n;
                }
                "xr" =>
                {
                    let mut bracket = 0;
                    let mut comma = 0;
                    for (i, c) in split.clone().next().unwrap().chars().enumerate()
                    {
                        match c
                        {
                            '(' | '{' | '[' => bracket += 1,
                            ')' | '}' | ']' => bracket -= 1,
                            ',' if bracket == 0 => comma = i,
                            _ =>
                            {}
                        }
                    }
                    if comma == 0
                    {
                        return Err("invalid x range");
                    }
                    let xr = split.next().unwrap().split_at(comma);
                    options.xr.0 = xr.0.parse::<f64>().expect("invalid xr");
                    options.xr.1 = xr.1[1..].parse::<f64>().expect("invalid xr")
                }
                "yr" =>
                {
                    let mut bracket = 0;
                    let mut comma = 0;
                    for (i, c) in split.clone().next().unwrap().chars().enumerate()
                    {
                        match c
                        {
                            '(' | '{' | '[' => bracket += 1,
                            ')' | '}' | ']' => bracket -= 1,
                            ',' if bracket == 0 => comma = i,
                            _ =>
                            {}
                        }
                    }
                    if comma == 0
                    {
                        return Err("invalid y range");
                    }
                    let yr = split.next().unwrap().split_at(comma);
                    options.yr.0 = yr.0.parse::<f64>().expect("invalid yr");
                    options.yr.1 = yr.1[1..].parse::<f64>().expect("invalid yr")
                }
                "zr" =>
                {
                    let mut bracket = 0;
                    let mut comma = 0;
                    for (i, c) in split.clone().next().unwrap().chars().enumerate()
                    {
                        match c
                        {
                            '(' | '{' | '[' => bracket += 1,
                            ')' | '}' | ']' => bracket -= 1,
                            ',' if bracket == 0 => comma = i,
                            _ =>
                            {}
                        }
                    }
                    if comma == 0
                    {
                        return Err("invalid z range");
                    }
                    let zr = split.next().unwrap().split_at(comma);
                    options.zr.0 = zr.0.parse::<f64>().expect("invalid zr");
                    options.zr.1 = zr.1[1..].parse::<f64>().expect("invalid zr")
                }
                "prec" | "precision" =>
                {
                    options.prec = match split.next().unwrap().parse::<u32>()
                    {
                        Ok(x) if x != 0 => (x, x),
                        _ => return Err("invalid prec"),
                    };
                }
                "graphprec" | "graphprecision" =>
                {
                    options.graph_prec = match split.next().unwrap().parse::<u32>()
                    {
                        Ok(x) if x != 0 => (x, x),
                        _ => return Err("invalid graphprec"),
                    };
                }
                "decimal" | "deci" | "decimals" =>
                {
                    let r = split.next().unwrap();
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
                        options.decimal_places = r.parse::<usize>().expect("invalid deci")
                    }
                }
                "multi" =>
                {
                    options.multi = split
                        .next()
                        .unwrap()
                        .parse::<bool>()
                        .expect("invalid multi")
                }
                "depth" =>
                {
                    options.depth = split
                        .next()
                        .unwrap()
                        .parse::<bool>()
                        .expect("invalid depth")
                }
                "flat" =>
                {
                    options.flat = split.next().unwrap().parse::<bool>().expect("invalid flat")
                }
                "small_e" =>
                {
                    options.small_e = split
                        .next()
                        .unwrap()
                        .parse::<bool>()
                        .expect("invalid small_e")
                }
                "tabbed" =>
                {
                    options.tabbed = split
                        .next()
                        .unwrap()
                        .parse::<bool>()
                        .expect("invalid tabbed")
                }
                "rt" =>
                {
                    options.real_time_output =
                        split.next().unwrap().parse::<bool>().expect("invalid rt")
                }
                "line" | "lines" =>
                {
                    options.lines = split.next().unwrap().parse::<bool>().expect("invalid line")
                }
                "polar" =>
                {
                    options.polar = split
                        .next()
                        .unwrap()
                        .parse::<bool>()
                        .expect("invalid polar")
                }
                "frac" =>
                {
                    options.frac = split.next().unwrap().parse::<bool>().expect("invalid frac")
                }
                "prompt" =>
                {
                    options.prompt = split
                        .next()
                        .unwrap()
                        .parse::<bool>()
                        .expect("invalid prompt")
                }
                "comma" =>
                {
                    options.comma = split
                        .next()
                        .unwrap()
                        .parse::<bool>()
                        .expect("invalid comma")
                }
                "graph" =>
                {
                    options.graph = split
                        .next()
                        .unwrap()
                        .parse::<bool>()
                        .expect("invalid graph")
                }
                "color" =>
                {
                    options.color = split
                        .next()
                        .unwrap()
                        .parse::<bool>()
                        .expect("invalid color")
                }
                "point" =>
                {
                    options.point_style = match split.next().unwrap().chars().next()
                    {
                        Some(x)
                            if matches!(
                                x,
                                '.' | '+'
                                    | 'x'
                                    | '*'
                                    | 's'
                                    | 'S'
                                    | 'o'
                                    | 'O'
                                    | 't'
                                    | 'T'
                                    | 'd'
                                    | 'D'
                                    | 'R'
                            ) =>
                        {
                            x
                        }
                        _ => return Err("invalid point"),
                    }
                }
                "sci" | "scientific" =>
                {
                    options.sci = split.next().unwrap().parse::<bool>().expect("invalid sci")
                }
                "base" =>
                {
                    options.base = match split.next().unwrap().parse::<usize>()
                    {
                        Ok(n) if (2..=36).contains(&n) => n,
                        _ => return Err("base out of range"),
                    }
                }
                "ticks" =>
                {
                    options.ticks = match split.next().unwrap().parse::<f64>()
                    {
                        Ok(n) => n,
                        _ => return Err("ticks out of range"),
                    }
                }
                "debug" =>
                {
                    options.debug = split
                        .next()
                        .unwrap()
                        .parse::<bool>()
                        .expect("invalid debug")
                }
                "rad" => options.deg = Radians,
                "deg" => options.deg = Degrees,
                "grad" => options.deg = Gradians,
                "tau" => options.tau = split.next().unwrap().parse::<bool>().expect("invalid tau"),
                _ =>
                {}
            }
        }
    }
    Ok(())
}
pub fn equal_to(options: Options, colors: &Colors, vars: &[Variable], l: &str, last: &str)
    -> String
{
    match l.replace(' ', "").as_str()
    {
        "colors" => format!(
            "{}textc={} {}promptc={} {}imagc={} {}scic={} \x1b[0mbracketc={} \x1b[38;2;{};{};{}mre1col={} \x1b[38;2;{};{};{}mim1col={} \x1b[38;2;{};{};{}mre2col={} \
             \x1b[38;2;{};{};{}mim2col={} \x1b[38;2;{};{};{}mre3col={} \x1b[38;2;{};{};{}mim3col={} \x1b[38;2;{};{};{}mre4col={} \x1b[38;2;{};{};{}mim4col={} \x1b[38;2;{};{};{}mre5col={} \
              \x1b[38;2;{};{};{}mim5col={} \x1b[38;2;{};{};{}mre6col={} \x1b[38;2;{};{};{}mim6col={}\x1b[0m",
            colors.text,
            &colors.text[2..],
            colors.prompt,
            &colors.prompt[2..],
            colors.imag,
            &colors.imag[2..],
            colors.sci,
            &colors.sci[2..],
            colors
                .brackets
                .iter()
                .fold(String::new(), |out, a| out + &format!("{}{},", a, &a[2..])).trim_end_matches(','),
            u8::from_str_radix(&colors.re1col[1..3],16).unwrap(),
            u8::from_str_radix(&colors.re1col[3..5],16).unwrap(),
            u8::from_str_radix(&colors.re1col[5..7],16).unwrap(),
            colors.re1col,
            u8::from_str_radix(&colors.im1col[1..3],16).unwrap(),
            u8::from_str_radix(&colors.im1col[3..5],16).unwrap(),
            u8::from_str_radix(&colors.im1col[5..7],16).unwrap(),
            colors.im1col,
            u8::from_str_radix(&colors.re2col[1..3],16).unwrap(),
            u8::from_str_radix(&colors.re2col[3..5],16).unwrap(),
            u8::from_str_radix(&colors.re2col[5..7],16).unwrap(),
            colors.re2col,
            u8::from_str_radix(&colors.im2col[1..3],16).unwrap(),
            u8::from_str_radix(&colors.im2col[3..5],16).unwrap(),
            u8::from_str_radix(&colors.im2col[5..7],16).unwrap(),
            colors.im2col,
            u8::from_str_radix(&colors.re3col[1..3],16).unwrap(),
            u8::from_str_radix(&colors.re3col[3..5],16).unwrap(),
            u8::from_str_radix(&colors.re3col[5..7],16).unwrap(),
            colors.re3col,
            u8::from_str_radix(&colors.im3col[1..3],16).unwrap(),
            u8::from_str_radix(&colors.im3col[3..5],16).unwrap(),
            u8::from_str_radix(&colors.im3col[5..7],16).unwrap(),
            colors.im3col,
            u8::from_str_radix(&colors.re4col[1..3],16).unwrap(),
            u8::from_str_radix(&colors.re4col[3..5],16).unwrap(),
            u8::from_str_radix(&colors.re4col[5..7],16).unwrap(),
            colors.re4col,
            u8::from_str_radix(&colors.im4col[1..3],16).unwrap(),
            u8::from_str_radix(&colors.im4col[3..5],16).unwrap(),
            u8::from_str_radix(&colors.im4col[5..7],16).unwrap(),
            colors.im4col,
            u8::from_str_radix(&colors.re5col[1..3],16).unwrap(),
            u8::from_str_radix(&colors.re5col[3..5],16).unwrap(),
            u8::from_str_radix(&colors.re5col[5..7],16).unwrap(),
            colors.re5col,
            u8::from_str_radix(&colors.im5col[1..3],16).unwrap(),
            u8::from_str_radix(&colors.im5col[3..5],16).unwrap(),
            u8::from_str_radix(&colors.im5col[5..7],16).unwrap(),
            colors.im5col,
            u8::from_str_radix(&colors.re6col[1..3],16).unwrap(),
            u8::from_str_radix(&colors.re6col[3..5],16).unwrap(),
            u8::from_str_radix(&colors.re6col[5..7],16).unwrap(),
            colors.re6col,
            u8::from_str_radix(&colors.im6col[1..3],16).unwrap(),
            u8::from_str_radix(&colors.im6col[3..5],16).unwrap(),
            u8::from_str_radix(&colors.im6col[5..7],16).unwrap(),
            colors.im6col,
        ),
        "color" => format!("{}", options.color),
        "depth" => format!("{}", options.depth),
        "flat" => format!("{}", options.flat),
        "prompt" => format!("{}", options.prompt),
        "rt" => format!("{}", options.real_time_output),
        "sci" | "scientific" => format!("{}", options.sci),
        "debug" => format!("{}", options.debug),
        "line" => format!("{}", options.lines),
        "polar" => format!("{}", options.polar),
        "frac" => format!("{}", options.frac),
        "multi" => format!("{}", options.multi),
        "tabbed" => format!("{}", options.tabbed),
        "comma" => format!("{}", options.comma),
        "graph" => format!("{}", options.graph),
        "point" => format!("{}", options.point_style),
        "base" => format!("{}", options.base),
        "ticks" => format!("{}", options.ticks),
        "decimal" | "deci" | "decimals" => format!("{}", options.decimal_places),
        "prec" | "precision" => format!("{}", options.prec.0),
        "graphprec" | "graphprecision" => format!("{}", options.graph_prec.0),
        "xr" => format!("{},{}", options.xr.0, options.xr.1),
        "yr" => format!("{},{}", options.yr.0, options.yr.1),
        "zr" => format!("{},{}", options.zr.0, options.zr.1),
        "range" => format!(
            "x:{},{} y:{},{} z:{},{}",
            options.xr.0, options.xr.1, options.yr.0, options.yr.1, options.zr.0, options.zr.1
        ),
        "frac_iter" => format!("{}", options.frac_iter),
        "2d" => format!("{}", options.samples_2d),
        "3d" => format!("{} {}", options.samples_3d.0, options.samples_3d.1),
        _ =>
        {
            let input=input_var(
                &l.replace('_', &format!("({})", last)),
                vars.to_vec(),
                &mut Vec::new(),
                &mut 0,
                options, false,
                &mut (false,0,0),
                true,
                0,
                Vec::new()
            );
            if let Ok(f)=input
            {
            parsed_to_string(&f.0 ,&options, colors)
        }
            else
            {
                String::new()
            }
        }
    }
}
pub fn parsed_to_string(input: &[NumStr], options: &Options, colors: &Colors) -> String
{
    let mut out = String::new();
    for i in input
    {
        match i
        {
            Num(n) =>
            {
                let n = get_output(*options, colors, n);
                out.push_str(&format!(
                    "{}{}{}",
                    n.0,
                    n.1,
                    if options.color { "\x1b[0m" } else { "" }
                ))
            }
            Vector(n) =>
            {
                let mut str = String::new();
                let mut num;
                for i in n
                {
                    num = get_output(*options, colors, i);
                    str.push_str(&format!(
                        "{}{}{},",
                        num.0,
                        num.1,
                        if options.color { "\x1b[0m" } else { "" }
                    ));
                }
                str.pop();
                out.push_str(&format!("{{{}}}", str))
            }
            Matrix(n) =>
            {
                let mut str = String::new();
                let mut num;
                for i in n
                {
                    for j in i
                    {
                        num = get_output(*options, colors, j);
                        str.push_str(&format!(
                            "{}{}{},",
                            num.0,
                            num.1,
                            if options.color { "\x1b[0m" } else { "" }
                        ));
                    }
                }
                str.pop();
                out.push_str(&format!("{{{}}}", str))
            }
            Str(n) => out.push_str(n),
        }
    }
    to_output(&out.chars().collect::<Vec<char>>(), options.color, colors)
}
pub fn set_commands(
    options: &mut Options,
    colors: &mut Colors,
    vars: &mut [Variable],
    l: &str,
    r: &str,
) -> Result<(), &'static str>
{
    match l
    {
        "re1col" => colors.re1col = r.to_string(),
        "im1col" => colors.im1col = r.to_string(),
        "re2col" => colors.re2col = r.to_string(),
        "im2col" => colors.im2col = r.to_string(),
        "re3col" => colors.re3col = r.to_string(),
        "im3col" => colors.im3col = r.to_string(),
        "re4col" => colors.re4col = r.to_string(),
        "im4col" => colors.im4col = r.to_string(),
        "re5col" => colors.re5col = r.to_string(),
        "im5col" => colors.im5col = r.to_string(),
        "re6col" => colors.re6col = r.to_string(),
        "im6col" => colors.im6col = r.to_string(),
        "textc" =>
        {
            match r.parse::<String>()
            {
                Ok(n) => colors.text = "\x1b[".to_owned() + &n,
                _ => return Err("Invalid col"),
            };
        }
        "promptc" =>
        {
            match r.parse::<String>()
            {
                Ok(n) => colors.prompt = "\x1b[".to_owned() + &n,
                _ => return Err("Invalid col"),
            };
        }
        "imagc" =>
        {
            match r.parse::<String>()
            {
                Ok(n) => colors.imag = "\x1b[".to_owned() + &n,
                _ => return Err("Invalid col"),
            };
        }
        "scic" =>
        {
            match r.parse::<String>()
            {
                Ok(n) => colors.sci = "\x1b[".to_owned() + &n,
                _ => return Err("Invalid col"),
            };
        }
        "bracketc" =>
        {
            match r.parse::<String>()
            {
                Ok(n) =>
                {
                    colors.brackets.clear();
                    for i in n.split(',')
                    {
                        colors.brackets.push("\x1b[".to_owned() + i)
                    }
                }
                _ => return Err("Invalid col"),
            };
        }
        "point" =>
        {
            let r = r.chars().next().unwrap();
            if matches!(
                r,
                '.' | '+' | 'x' | '*' | 's' | 'S' | 'o' | 'O' | 't' | 'T' | 'd' | 'D' | 'r' | 'R'
            )
            {
                options.point_style = r;
            }
            else
            {
                return Err("Invalid point type");
            }
        }
        "base" =>
        {
            match r.parse::<usize>()
            {
                Ok(n) if (2..=36).contains(&n) => options.base = n,
                _ => return Err("Invalid base"),
            };
        }
        "ticks" =>
        {
            match r.parse::<f64>()
            {
                Ok(n) => options.ticks = n,
                _ => return Err("Invalid base"),
            };
        }
        "decimal" | "deci" | "decimals" =>
        {
            match do_math(
                input_var(
                    r,
                    vars.to_vec(),
                    &mut Vec::new(),
                    &mut 0,
                    *options,
                    false,
                    &mut (false, 0, 0),
                    false,
                    0,
                    Vec::new(),
                )?
                .0,
                *options,
                Vec::new(),
            )?
            .num()?
            .real()
            .to_f64() as isize
            {
                n if n == -1 => options.decimal_places = usize::MAX - 1,
                n if n == -2 => options.decimal_places = usize::MAX,
                n if n >= 0 => options.decimal_places = n as usize,
                _ => return Err("Invalid decimal"),
            };
        }
        "graphprec" | "graphprecision" => match do_math(
            input_var(
                r,
                vars.to_vec(),
                &mut Vec::new(),
                &mut 0,
                *options,
                false,
                &mut (false, 0, 0),
                false,
                0,
                Vec::new(),
            )?
            .0,
            *options,
            Vec::new(),
        )?
        .num()?
        .real()
        .to_f64() as u32
        {
            n if n != 0 =>
            {
                options.graph_prec = (n, n);
            }
            _ => return Err("Invalid graphprecision"),
        },
        "prec" | "precision" => match do_math(
            input_var(
                r,
                vars.to_vec(),
                &mut Vec::new(),
                &mut 0,
                *options,
                false,
                &mut (false, 0, 0),
                false,
                0,
                Vec::new(),
            )?
            .0,
            *options,
            Vec::new(),
        )?
        .num()?
        .real()
        .to_f64() as u32
        {
            n if n != 0 =>
            {
                options.prec = (n, n);
                if !vars.is_empty()
                {
                    let v = get_vars(*options);
                    for var in vars.iter_mut()
                    {
                        for i in &v
                        {
                            if i.name == var.name && i.unparsed == var.unparsed
                            {
                                *var = i.clone();
                            }
                        }
                    }
                    let mut redef = Vec::new();
                    for (i, var) in vars.to_vec().iter().enumerate()
                    {
                        if !var.unparsed.is_empty()
                        {
                            let mut func_vars: Vec<(isize, String)> = Vec::new();
                            if var.name.contains(&'(')
                            {
                                let mut l = var.name.clone();
                                l.drain(0..=l.iter().position(|c| c == &'(').unwrap());
                                l.pop();
                                for i in l.split(|c| c == &',')
                                {
                                    func_vars.push((-1, i.iter().collect()));
                                }
                            }
                            redef.push(var.name.clone());
                            let parsed = input_var(
                                &var.unparsed,
                                vars.to_vec(),
                                &mut func_vars,
                                &mut 0,
                                *options,
                                false,
                                &mut (false, 0, 0),
                                false,
                                0,
                                Vec::new(),
                            )
                            .unwrap()
                            .0;
                            vars[i].parsed = if l.contains('(')
                            {
                                parsed
                            }
                            else
                            {
                                vec![do_math(parsed, *options, Vec::new())
                                    .unwrap_or(Num(Complex::new(options.prec)))]
                            };
                        }
                    }
                    let mut k = 0;
                    while k < redef.len()
                    {
                        for (j, v) in vars.to_vec().iter().enumerate()
                        {
                            if redef[k] != v.name
                                && v.unparsed.contains(
                                    &redef[k][0..=redef[k]
                                        .iter()
                                        .position(|a| a == &'(')
                                        .unwrap_or(redef[k].len() - 1)],
                                )
                            {
                                let mut func_vars: Vec<(isize, String)> = Vec::new();
                                if v.name.contains(&'(')
                                {
                                    let mut l = v.name.clone();
                                    l.drain(0..=l.iter().position(|c| c == &'(').unwrap());
                                    l.pop();
                                    for i in l.split(|c| c == &',')
                                    {
                                        func_vars.push((-1, i.iter().collect()));
                                    }
                                }
                                let parsed = input_var(
                                    &v.unparsed.clone(),
                                    vars.to_vec(),
                                    &mut func_vars,
                                    &mut 0,
                                    *options,
                                    false,
                                    &mut (false, 0, 0),
                                    false,
                                    0,
                                    Vec::new(),
                                )
                                .unwrap()
                                .0;
                                let check = vars[j].parsed.clone();
                                vars[j].parsed = if l.contains('(')
                                {
                                    parsed
                                }
                                else
                                {
                                    vec![do_math(parsed, *options, Vec::new())
                                        .unwrap_or(Num(Complex::new(options.prec)))]
                                };
                                if check != vars[j].parsed
                                {
                                    redef.push(v.name.clone());
                                }
                            }
                        }
                        k += 1;
                    }
                }
            }
            _ => return Err("Invalid precision"),
        },
        "range" =>
        {
            if r.contains(',')
            {
                let mut bracket = 0;
                let mut comma = 0;
                for (i, c) in r.chars().enumerate()
                {
                    match c
                    {
                        '(' | '{' | '[' => bracket += 1,
                        ')' | '}' | ']' => bracket -= 1,
                        ',' if bracket == 0 => comma = i,
                        _ =>
                        {}
                    }
                }
                let (min, max) = (
                    do_math(
                        input_var(
                            r.split_at(comma).0,
                            vars.to_vec(),
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                            false,
                            &mut (false, 0, 0),
                            false,
                            0,
                            Vec::new(),
                        )?
                        .0,
                        *options,
                        Vec::new(),
                    )?
                    .num()?
                    .real()
                    .to_f64(),
                    do_math(
                        input_var(
                            r.split_at(comma).1,
                            vars.to_vec(),
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                            false,
                            &mut (false, 0, 0),
                            false,
                            0,
                            Vec::new(),
                        )?
                        .0,
                        *options,
                        Vec::new(),
                    )?
                    .num()?
                    .real()
                    .to_f64(),
                );
                (
                    options.xr.0,
                    options.xr.1,
                    options.yr.0,
                    options.yr.1,
                    options.zr.0,
                    options.zr.1,
                ) = (min, max, min, max, min, max)
            }
            else
            {
                let n = do_math(
                    input_var(
                        r,
                        vars.to_vec(),
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                        Vec::new(),
                    )?
                    .0,
                    *options,
                    Vec::new(),
                )?
                .num()?
                .real()
                .to_f64();
                (
                    options.xr.0,
                    options.xr.1,
                    options.yr.0,
                    options.yr.1,
                    options.zr.0,
                    options.zr.1,
                ) = (-n, n, -n, n, -n, n)
            }
        }
        "xr" =>
        {
            if r.contains(',')
            {
                let mut bracket = 0;
                let mut comma = 0;
                for (i, c) in r.chars().enumerate()
                {
                    match c
                    {
                        '(' | '{' | '[' => bracket += 1,
                        ')' | '}' | ']' => bracket -= 1,
                        ',' if bracket == 0 => comma = i,
                        _ =>
                        {}
                    }
                }
                options.xr.0 = do_math(
                    input_var(
                        r.split_at(comma).0,
                        vars.to_vec(),
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                        Vec::new(),
                    )?
                    .0,
                    *options,
                    Vec::new(),
                )?
                .num()?
                .real()
                .to_f64();
                options.xr.1 = do_math(
                    input_var(
                        r.split_at(comma).1,
                        vars.to_vec(),
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                        Vec::new(),
                    )?
                    .0,
                    *options,
                    Vec::new(),
                )?
                .num()?
                .real()
                .to_f64();
            }
            else
            {
                let n = do_math(
                    input_var(
                        r,
                        vars.to_vec(),
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                        Vec::new(),
                    )?
                    .0,
                    *options,
                    Vec::new(),
                )?
                .num()?
                .real()
                .to_f64();
                options.xr = (-n, n)
            }
        }
        "yr" =>
        {
            if r.contains(',')
            {
                let mut bracket = 0;
                let mut comma = 0;
                for (i, c) in r.chars().enumerate()
                {
                    match c
                    {
                        '(' | '{' | '[' => bracket += 1,
                        ')' | '}' | ']' => bracket -= 1,
                        ',' if bracket == 0 => comma = i,
                        _ =>
                        {}
                    }
                }
                options.yr.0 = do_math(
                    input_var(
                        r.split_at(comma).0,
                        vars.to_vec(),
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                        Vec::new(),
                    )?
                    .0,
                    *options,
                    Vec::new(),
                )?
                .num()?
                .real()
                .to_f64();
                options.yr.1 = do_math(
                    input_var(
                        r.split_at(comma).1,
                        vars.to_vec(),
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                        Vec::new(),
                    )?
                    .0,
                    *options,
                    Vec::new(),
                )?
                .num()?
                .real()
                .to_f64();
            }
            else
            {
                let n = do_math(
                    input_var(
                        r,
                        vars.to_vec(),
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                        Vec::new(),
                    )?
                    .0,
                    *options,
                    Vec::new(),
                )?
                .num()?
                .real()
                .to_f64();
                options.yr = (-n, n)
            }
        }
        "zr" =>
        {
            if r.contains(',')
            {
                let mut bracket = 0;
                let mut comma = 0;
                for (i, c) in r.chars().enumerate()
                {
                    match c
                    {
                        '(' | '{' | '[' => bracket += 1,
                        ')' | '}' | ']' => bracket -= 1,
                        ',' if bracket == 0 => comma = i,
                        _ =>
                        {}
                    }
                }
                options.zr.0 = do_math(
                    input_var(
                        r.split_at(comma).0,
                        vars.to_vec(),
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                        Vec::new(),
                    )?
                    .0,
                    *options,
                    Vec::new(),
                )?
                .num()?
                .real()
                .to_f64();
                options.zr.1 = do_math(
                    input_var(
                        r.split_at(comma).1,
                        vars.to_vec(),
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                        Vec::new(),
                    )?
                    .0,
                    *options,
                    Vec::new(),
                )?
                .num()?
                .real()
                .to_f64();
            }
            else
            {
                let n = do_math(
                    input_var(
                        r,
                        vars.to_vec(),
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                        Vec::new(),
                    )?
                    .0,
                    *options,
                    Vec::new(),
                )?
                .num()?
                .real()
                .to_f64();
                options.zr = (-n, n)
            }
        }
        "frac_iter" =>
        {
            match r.parse::<usize>()
            {
                Ok(n) => options.frac_iter = n,
                _ => return Err("Invalid frac_iter"),
            };
        }
        "2d" =>
        {
            options.samples_2d = do_math(
                input_var(
                    r,
                    vars.to_vec(),
                    &mut Vec::new(),
                    &mut 0,
                    *options,
                    false,
                    &mut (false, 0, 0),
                    false,
                    0,
                    Vec::new(),
                )?
                .0,
                *options,
                Vec::new(),
            )?
            .num()?
            .real()
            .to_f64() as usize
        }
        "3d" =>
        {
            if r.contains(',')
            {
                options.samples_3d = (
                    do_math(
                        input_var(
                            r.split(',').next().unwrap(),
                            vars.to_vec(),
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                            false,
                            &mut (false, 0, 0),
                            false,
                            0,
                            Vec::new(),
                        )?
                        .0,
                        *options,
                        Vec::new(),
                    )?
                    .num()?
                    .real()
                    .to_f64() as usize,
                    do_math(
                        input_var(
                            r.split(',').last().unwrap(),
                            vars.to_vec(),
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                            false,
                            &mut (false, 0, 0),
                            false,
                            0,
                            Vec::new(),
                        )?
                        .0,
                        *options,
                        Vec::new(),
                    )?
                    .num()?
                    .real()
                    .to_f64() as usize,
                );
            }
            else
            {
                let n = do_math(
                    input_var(
                        r,
                        vars.to_vec(),
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                        Vec::new(),
                    )?
                    .0,
                    *options,
                    Vec::new(),
                )?
                .num()?
                .real()
                .to_f64() as usize;
                options.samples_3d = (n, n)
            }
        }
        _ => return Ok(()),
    }
    Err("")
}
pub fn silent_commands(options: &mut Options, input: &[char])
{
    match input.iter().collect::<String>().as_str()
    {
        "default" | "defaults" | "reset" => *options = Options::default(),
        "color" => options.color = !options.color,
        "prompt" => options.prompt = !options.prompt,
        "depth" => options.depth = !options.depth,
        "flat" => options.flat = !options.flat,
        "deg" => options.deg = Degrees,
        "rad" => options.deg = Radians,
        "grad" => options.deg = Gradians,
        "rt" => options.real_time_output = !options.real_time_output,
        "tau" => options.tau = true,
        "pi" => options.tau = false,
        "small_e" => options.small_e = !options.small_e,
        "sci" | "scientific" => options.sci = !options.sci,
        "line" | "lines" => options.lines = !options.lines,
        "polar" => options.polar = !options.polar,
        "frac" => options.frac = !options.frac,
        "multi" => options.multi = !options.multi,
        "tabbed" => options.tabbed = !options.tabbed,
        "comma" => options.comma = !options.comma,
        "graph" => options.graph = !options.graph,
        _ =>
        {}
    }
}
#[allow(clippy::too_many_arguments)]
pub fn commands(
    options: &mut Options,
    colors: &Colors,
    vars: &mut [Variable],
    lines: &[String],
    input: &[char],
    stdout: &mut Stdout,
)
{
    match input.iter().collect::<String>().as_str()
    {
        "default" | "defaults" | "reset" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            *options = Options::default();
        }
        "color" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.color = !options.color;
        }
        "prompt" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.prompt = !options.prompt;
        }
        "depth" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.depth = !options.depth;
        }
        "flat" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.flat = !options.flat;
        }
        "deg" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.deg = Degrees;
        }
        "rad" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.deg = Radians;
        }
        "grad" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.deg = Gradians;
        }
        "rt" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.real_time_output = !options.real_time_output;
        }
        "tau" => options.tau = true,
        "pi" => options.tau = false,
        "small_e" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.small_e = !options.small_e;
        }
        "sci" | "scientific" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.sci = !options.sci;
        }
        "clear" =>
        {
            execute!(io::stdout(), Clear(ClearType::Purge)).unwrap();
            print!("\x1b[H\x1b[J{}", prompt(*options, colors));
            stdout.flush().unwrap();
        }
        "debug" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.debug = !options.debug;
        }
        "help" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            help();
        }
        "line" | "lines" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.lines = !options.lines;
        }
        "polar" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.polar = !options.polar;
        }
        "frac" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.frac = !options.frac;
        }
        "multi" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.multi = !options.multi;
        }
        "tabbed" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.tabbed = !options.tabbed;
        }
        "comma" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.comma = !options.comma;
        }
        "graph" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.graph = !options.graph;
        }
        "history" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            for l in lines
            {
                print!("{}\n\x1b[G", l);
            }
            stdout.flush().unwrap();
        }
        "vars" | "variables" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            for v in vars.iter()
            {
                if v.name.contains(&'(')
                {
                    let mut func_vars: Vec<(isize, String)> = Vec::new();

                    let mut l = v.name.clone();
                    l.drain(0..=l.iter().position(|c| c == &'(').unwrap());
                    l.pop();
                    for i in l.split(|c| c == &',')
                    {
                        func_vars.push((-1, i.iter().collect()));
                    }

                    print!(
                        "{}={}\n\x1b[G",
                        v.name.iter().collect::<String>(),
                        parsed_to_string(
                            &match input_var(
                                &v.unparsed,
                                vars.to_vec(),
                                &mut func_vars,
                                &mut 0,
                                *options,
                                false,
                                &mut (false, 0, 0),
                                true,
                                0,
                                Vec::new()
                            )
                            {
                                Ok(n) => n.0,
                                _ => v.parsed.clone(),
                            },
                            options,
                            colors
                        )
                    );
                }
                else
                {
                    match &v.parsed[0]
                    {
                        Num(n) =>
                        {
                            let n = get_output(*options, colors, n);
                            print!(
                                "{}={}{}\n\x1b[G",
                                v.name.iter().collect::<String>(),
                                n.0,
                                n.1
                            )
                        }
                        Vector(m) =>
                        {
                            let mut st = String::new();
                            for i in m
                            {
                                let n = get_output(*options, colors, i);
                                st.push_str(&n.0);
                                st.push_str(&n.1);
                                st.push(',');
                            }
                            print!(
                                "{}={{{}}}\n\x1b[G",
                                v.name.iter().collect::<String>(),
                                st.trim_end_matches(',')
                            )
                        }
                        Matrix(m) =>
                        {
                            let mut st = String::new();
                            for i in m
                            {
                                st.push('{');
                                for g in i
                                {
                                    let n = get_output(*options, colors, g);
                                    st.push_str(&n.0);
                                    st.push_str(&n.1);
                                    st.push(',');
                                }
                                st = st.trim_end_matches(',').to_string();
                                st.push('}');
                                st.push(',');
                            }
                            print!(
                                "{}={{{}}}\n\x1b[G",
                                v.name.iter().collect::<String>(),
                                st.trim_end_matches(',')
                            )
                        }
                        _ => continue,
                    }
                }
            }
            stdout.flush().unwrap();
        }
        "lvars" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            for v in vars.iter()
            {
                print!(
                    "{}={}\n\x1b[G",
                    v.name.iter().collect::<String>(),
                    parsed_to_string(&v.parsed, options, colors)
                );
            }
            stdout.flush().unwrap();
        }
        "version" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        }
        "exit" | "quit" | "break" =>
        {
            print!("\x1b[A\x1b[G\x1b[J");
            stdout.flush().unwrap();
            terminal::disable_raw_mode().unwrap();
            std::process::exit(0);
        }
        _ =>
        {
            let n = (*input).iter().collect::<String>();
            let mut split = n.splitn(2, ' ');
            let next = split.next().unwrap();
            if next == "history"
            {
                print!("\x1b[A\x1b[G\x1b[K");
                let r = split.next().unwrap();
                for i in lines
                {
                    if i.contains(r)
                    {
                        print!("{}\n\x1b[G", i);
                    }
                }
                stdout.flush().unwrap();
            }
        }
    }
}