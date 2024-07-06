use crate::{
    complex::NumStr::{Matrix, Num, Vector},
    help::{help, help_for},
    load_vars::get_vars,
    math::do_math,
    misc::{insert_last, parsed_to_string, to_output},
    parse::input_var,
    print::{custom_units, get_output},
    AngleType::{Degrees, Gradians, Radians},
    Auto, Colors, GraphType, HowGraphing,
    Notation::{LargeEngineering, Normal, Scientific, SmallEngineering},
    Number, Options, Variable,
};
use crossterm::{
    execute, terminal,
    terminal::{Clear, ClearType},
};
use rug::{float::Special::Nan, Complex, Float};
use std::{
    fs::File,
    io::{BufRead, BufReader, Stdout, Write},
    process,
};
pub fn arg_opts(
    options: &mut Options,
    colors: &mut Colors,
    args: &mut Vec<String>,
    vars: &[Variable],
    soft: bool,
) -> Result<bool, &'static str>
{
    let mut default = false;
    if soft
    {
        args.remove(0);
    }
    loop
    {
        if args.is_empty()
        {
            break;
        }
        match args[0].as_str()
        {
            "-i" =>
            {
                options.stay_interactive = !options.stay_interactive;
                args.remove(0);
            }
            "-v" | "--version" =>
            {
                println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                process::exit(0)
            }
            "-h" | "--help" =>
            {
                if args.len() > 1
                {
                    println!("{}", help_for(&args[1..].join(" ")));
                }
                else
                {
                    help();
                }
                process::exit(0)
            }
            "--default" =>
            {
                args.remove(0);
                *options = Options::default();
                default = true;
            }
            "--" =>
            {
                args.remove(0);
                break;
            }
            _ if !args[0].starts_with("--") =>
            {
                break;
            }
            _ =>
            {
                let arg = args[0].trim_start_matches('-');
                let mut split = arg.splitn(2, '=');
                if split.clone().count() == 2
                {
                    match set_commands(
                        options,
                        colors,
                        &mut vars.to_vec(),
                        split.next().unwrap(),
                        split.next().unwrap(),
                    )
                    {
                        Err(s) if !s.is_empty() =>
                        {
                            if soft
                            {
                                break;
                            }
                            else
                            {
                                return Err(s);
                            }
                        }
                        Ok(()) =>
                        {
                            if soft
                            {
                                break;
                            }
                            else
                            {
                                println!("{} failed", arg);
                                process::exit(1);
                            }
                        }
                        _ =>
                        {}
                    }
                }
                else if !silent_commands(options, &arg.chars().collect::<Vec<char>>())
                {
                    if soft
                    {
                        break;
                    }
                    else
                    {
                        println!("{} failed", args[0]);
                        process::exit(1);
                    }
                }
                args.remove(0);
            }
        }
    }
    Ok(default)
}
pub fn file_opts(
    options: &mut Options,
    colors: &mut Colors,
    file_path: &String,
    vars: &[Variable],
    check: Vec<usize>,
    soft: bool,
) -> Result<Vec<usize>, &'static str>
{
    let mut err = Vec::new();
    if let Ok(file) = File::open(file_path)
    {
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().map(|l| l.unwrap()).enumerate()
        {
            if !(check.is_empty() || check.iter().any(|n| n == &i))
            {
                continue;
            }
            if line.starts_with('#') || line.is_empty()
            {
                continue;
            }
            let mut split = line.splitn(2, '=');
            if split.clone().count() == 2
            {
                match set_commands(
                    options,
                    colors,
                    &mut vars.to_vec(),
                    split.next().unwrap(),
                    split.next().unwrap(),
                )
                {
                    Err(s) if !s.is_empty() =>
                    {
                        if soft
                        {
                            err.push(i)
                        }
                        else
                        {
                            return Err(s);
                        }
                    }
                    Ok(()) =>
                    {
                        if soft
                        {
                            err.push(i);
                        }
                        else
                        {
                            println!("{} failed", line);
                            process::exit(1);
                        }
                    }
                    _ =>
                    {}
                }
            }
            else if !silent_commands(options, &line.chars().collect::<Vec<char>>())
            {
                if soft
                {
                    err.push(i);
                }
                else
                {
                    println!("{} failed", line);
                    process::exit(1);
                }
            }
        }
    }
    Ok(err)
}
pub fn set_commands(
    options: &mut Options,
    colors: &mut Colors,
    vars: &mut [Variable],
    l: &str,
    o: &str,
) -> Result<(), &'static str>
{
    let s = o.replace(" ", "");
    let r = s.as_str();
    match l
    {
        "color" | "colour" =>
        {
            options.color = match r
            {
                "1" | "true" | "True" | "always" => Auto::True,
                "Auto" | "auto" => Auto::Auto,
                "0" | "false" | "False" | "never" => Auto::False,
                _ => return Err("not true/false/auto"),
            }
        }
        "line" | "lines" =>
        {
            options.lines = match r
            {
                "1" | "true" | "True" | "always" => Auto::True,
                "Auto" | "auto" => Auto::Auto,
                "0" | "false" | "False" | "never" => Auto::False,
                _ => return Err("not true/false/auto"),
            }
        }
        "angle" =>
        {
            options.angle = match r
            {
                "deg" | "degree" | "degrees" => Degrees,
                "rad" | "radians" | "radian" => Radians,
                "grad" | "gradians" | "gradian" => Gradians,
                _ => return Err("bad angle type"),
            }
        }
        "notation" =>
        {
            options.notation = match r
            {
                "sci" | "scientific" | "s" | "10^" | "*10^" | "10" => Scientific,
                "engSmall" | "e" => SmallEngineering,
                "engLarge" | "eng" | "engineering" | "E" => LargeEngineering,
                "normal" | "n" => Normal,
                _ => return Err("bad notation type"),
            }
        }
        "graph" =>
        {
            options.graphtype = match r
            {
                "normal" | "true" => GraphType::Normal,
                "null" | "none" | "false" => GraphType::None,
                "depth" => GraphType::Depth,
                "flat" => GraphType::Flat,
                "domain" | "domain1" | "domain_coloring" => GraphType::Domain,
                "domain_alt" | "domainalt" | "domain2" => GraphType::DomainAlt,
                _ => return Err("bad graph type"),
            }
        }
        "saveto" =>
        {
            if r == "null"
            {
                colors.graphtofile.clear()
            }
            else
            {
                colors.graphtofile = r.to_string()
            }
        }
        "recol" =>
        {
            colors.recol = r
                .split(',')
                .map(|a| {
                    if a.len() == 6 && a.chars().all(|c| c.is_ascii_hexdigit())
                    {
                        Some("#".to_owned() + a)
                    }
                    else
                    {
                        None
                    }
                })
                .take_while(|a| a.is_some())
                .map(|a| a.unwrap())
                .collect::<Vec<String>>()
        }
        "imcol" =>
        {
            colors.imcol = r
                .split(',')
                .map(|a| {
                    if a.len() == 6 && a.chars().all(|c| c.is_ascii_hexdigit())
                    {
                        Some("#".to_owned() + a)
                    }
                    else
                    {
                        None
                    }
                })
                .take_while(|a| a.is_some())
                .map(|a| a.unwrap())
                .collect::<Vec<String>>()
        }
        "textc" => colors.text = "\x1b[".to_owned() + r,
        "promptc" => colors.prompt = "\x1b[".to_owned() + r,
        "imagc" => colors.imag = "\x1b[".to_owned() + r,
        "scic" => colors.sci = "\x1b[".to_owned() + r,
        "unitsc" => colors.units = "\x1b[".to_owned() + r,
        "bracketc" =>
        {
            if r == "null"
            {
                colors.brackets.clear()
            }
            else
            {
                colors.brackets = r
                    .split(',')
                    .map(|a| "\x1b[".to_owned() + a)
                    .collect::<Vec<String>>()
            }
        }
        "default_units" =>
        {
            if r == "null"
            {
                colors.default_units.clear()
            }
            else
            {
                let n = Number::from(Complex::with_val(options.prec, Nan), None);
                colors.default_units = r
                    .split(',')
                    .map(|a| {
                        (a.to_string(), {
                            let i = input_var(
                                a,
                                vars,
                                &mut Vec::new(),
                                &mut 0,
                                *options,
                                false,
                                0,
                                Vec::new(),
                                false,
                                &mut Vec::new(),
                                None,
                            )
                            .unwrap_or((
                                Vec::new(),
                                Vec::new(),
                                HowGraphing::default(),
                                false,
                                None,
                            ));
                            do_math(i.0, *options, i.1)
                                .unwrap_or(Num(n.clone()))
                                .num()
                                .unwrap_or(n.clone())
                        })
                    })
                    .collect::<Vec<(String, Number)>>()
            }
        }
        "label" =>
        {
            let mut split = o.split(',');
            if split.clone().count() == 2
            {
                colors.label.0 = split.next().unwrap().to_string();
                colors.label.1 = split.next().unwrap().to_string();
            }
            else if split.clone().count() == 3
            {
                colors.label = (
                    split.next().unwrap().to_string(),
                    split.next().unwrap().to_string(),
                    split.next().unwrap().to_string(),
                )
            }
        }
        "point" | "points" =>
        {
            if r.is_empty()
            {
                return Err("Invalid point type");
            }
            else
            {
                let r = r.chars().next().unwrap();
                options.point_style = match r
                {
                    '.' => 0,
                    '+' => 1,
                    'x' => 2,
                    '*' => 3,
                    's' => 4,
                    'S' => 5,
                    'o' => 6,
                    'O' => 7,
                    't' => 8,
                    'T' => 9,
                    'd' => 10,
                    'D' => 11,
                    'r' => 12,
                    'R' => 13,
                    _ => return Err("invalid point type"),
                }
            }
        }
        "graphcli" | "slowcheck" | "interactive" | "prompt" | "surface" | "rt" | "siunits"
        | "keepzeros" | "polar" | "frac" | "fractions" | "fractionsv" | "fractionsm" | "multi"
        | "tabbed" | "comma" | "units" | "scalegraph" | "debug" | "vars" | "onaxis" | "base"
        | "ticks" | "decimal" | "deci" | "decimals" | "graphprec" | "graphprecision" | "prec"
        | "windowsize" | "precision" | "range" | "xr" | "yr" | "zr" | "vrange" | "vxr" | "vyr"
        | "vzr" | "2d" | "3d" | "progress" =>
        {
            let mut args: Vec<Float> = Vec::new();
            {
                let mut bracket = 0;
                let mut last = 0;
                let oc = o.chars().collect::<Vec<char>>();
                for (i, c) in oc.iter().enumerate()
                {
                    match c
                    {
                        '(' | '{' | '[' => bracket += 1,
                        ')' | '}' | ']' => bracket -= 1,
                        ',' if bracket == 0 =>
                        {
                            let parsed = input_var(
                                &oc[last..i].iter().collect::<String>(),
                                vars,
                                &mut Vec::new(),
                                &mut 0,
                                *options,
                                false,
                                0,
                                Vec::new(),
                                false,
                                &mut Vec::new(),
                                None,
                            )?;
                            args.push(
                                do_math(parsed.0, *options, parsed.1)?
                                    .num()?
                                    .number
                                    .real()
                                    .clone(),
                            );
                            last = i + 1;
                        }
                        _ =>
                        {}
                    }
                }
                let parsed = input_var(
                    &o[last..],
                    vars,
                    &mut Vec::new(),
                    &mut 0,
                    *options,
                    false,
                    0,
                    Vec::new(),
                    false,
                    &mut Vec::new(),
                    None,
                )?;
                args.push(
                    do_math(parsed.0, *options, parsed.1)?
                        .num()?
                        .number
                        .real()
                        .clone(),
                );
            }
            match l
            {
                "keep_data_file" => options.keep_data_file = args[0] != 0.0,
                "graphcli" => options.graph_cli = args[0] != 0.0,
                "interactive" => options.stay_interactive = args[0] != 0.0,
                "prompt" => options.prompt = args[0] != 0.0,
                "surface" => options.surface = args[0] != 0.0,
                "rt" => options.real_time_output = args[0] != 0.0,
                "progress" => options.progress = args[0] != 0.0,
                "siunits" => options.si_units = args[0] != 0.0,
                "keepzeros" => options.keep_zeros = args[0] != 0.0,
                "polar" => options.polar = args[0] != 0.0,
                "frac" | "fractions" => options.frac.num = args[0] != 0.0,
                "fractionsv" => options.frac.vec = args[0] != 0.0,
                "fractionsm" => options.frac.mat = args[0] != 0.0,
                "multi" => options.multi = args[0] != 0.0,
                "tabbed" => options.tabbed = args[0] != 0.0,
                "comma" => options.comma = args[0] != 0.0,
                "units" => options.units = args[0] != 0.0,
                "scalegraph" => options.scale_graph = args[0] != 0.0,
                "debug" => options.debug = args[0] != 0.0,
                "vars" => options.allow_vars = args[0] != 0.0,
                "onaxis" => options.onaxis = args[0] != 0.0,
                "base" =>
                {
                    if args.len() == 2
                    {
                        let n1 = args[0]
                            .to_integer()
                            .unwrap_or_default()
                            .to_i32()
                            .unwrap_or_default();
                        let n2 = args[1]
                            .to_integer()
                            .unwrap_or_default()
                            .to_i32()
                            .unwrap_or_default();
                        if (2..=36).contains(&n1) && (2..=36).contains(&n2)
                        {
                            options.base = (n1, n2)
                        }
                        else
                        {
                            return Err("out of range of 2..=36");
                        }
                    }
                    else
                    {
                        let n = args[0]
                            .to_integer()
                            .unwrap_or_default()
                            .to_i32()
                            .unwrap_or_default();
                        if (2..=36).contains(&n)
                        {
                            options.base = (n, n)
                        }
                        else
                        {
                            return Err("out of range of 2..=36");
                        }
                    }
                }
                "ticks" =>
                {
                    if args.len() == 3
                    {
                        options.ticks = (args[0].to_f64(), args[1].to_f64(), args[2].to_f64())
                    }
                    else if args.len() == 2
                    {
                        (options.ticks.0, options.ticks.1) = (args[0].to_f64(), args[1].to_f64())
                    }
                    else
                    {
                        let n = args[0].to_f64();
                        options.ticks = (n, n, n);
                    }
                }
                "slowcheck" =>
                {
                    options.slowcheck = args[0]
                        .to_integer()
                        .unwrap_or_default()
                        .to_u128()
                        .unwrap_or_default()
                }
                "decimal" | "deci" | "decimals" =>
                {
                    options.decimal_places = match args[0]
                        .to_integer()
                        .unwrap_or_default()
                        .to_isize()
                        .unwrap_or_default()
                    {
                        -1 => usize::MAX - 1,
                        -2 => usize::MAX,
                        n if n >= 0 => n as usize,
                        _ => return Err("Invalid decimal"),
                    };
                }
                "graphprec" | "graphprecision" => match args[0]
                    .to_integer()
                    .unwrap_or_default()
                    .to_u32()
                    .unwrap_or_default()
                {
                    n if n != 0 => options.graph_prec = n,
                    _ => return Err("Invalid graphprecision"),
                },
                "windowsize" =>
                {
                    if args.len() == 1
                    {
                        options.window_size = (
                            args[0]
                                .to_integer()
                                .unwrap_or_default()
                                .to_usize()
                                .unwrap_or_default(),
                            args[0]
                                .to_integer()
                                .unwrap_or_default()
                                .to_usize()
                                .unwrap_or_default(),
                        )
                    }
                    else
                    {
                        options.window_size = (
                            args[0]
                                .to_integer()
                                .unwrap_or_default()
                                .to_usize()
                                .unwrap_or_default(),
                            args[1]
                                .to_integer()
                                .unwrap_or_default()
                                .to_usize()
                                .unwrap_or_default(),
                        )
                    }
                }
                "prec" | "precision" => match args[0]
                    .to_integer()
                    .unwrap_or_default()
                    .to_u32()
                    .unwrap_or_default()
                {
                    n if n != 0 =>
                    {
                        options.prec = n;
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
                                    let mut fvs = Vec::new();
                                    let mut unparsed = var.unparsed.clone();
                                    if unparsed.contains(':')
                                    {
                                        let un = unparsed;
                                        let mut split = un.split(':').collect::<Vec<&str>>();
                                        unparsed = split.pop().unwrap().to_string();
                                        for i in split
                                        {
                                            if i.contains('=')
                                            {
                                                let mut split = i.splitn(2, '=');
                                                let s = split.next().unwrap().to_string();
                                                let parsed = input_var(
                                                    split.next().unwrap(),
                                                    vars,
                                                    &mut func_vars,
                                                    &mut 0,
                                                    *options,
                                                    false,
                                                    0,
                                                    s.chars().collect::<Vec<char>>(),
                                                    false,
                                                    &mut Vec::new(),
                                                    None,
                                                )?;
                                                func_vars.push((-1, s.clone()));
                                                fvs.push((s, parsed.0));
                                                fvs.extend(parsed.1)
                                            }
                                        }
                                    }
                                    let mut parsed = match input_var(
                                        &unparsed,
                                        vars,
                                        &mut func_vars,
                                        &mut 0,
                                        *options,
                                        false,
                                        0,
                                        var.name.clone(),
                                        false,
                                        &mut Vec::new(),
                                        None,
                                    )
                                    {
                                        Ok(n) => (n.0, n.1),
                                        _ => return Err("prec crash"),
                                    };
                                    parsed.1.extend(fvs);
                                    if var.name.contains(&'(')
                                        && var
                                            .unparsed
                                            .contains(var.name.split(|c| c == &'(').next().unwrap())
                                        && (var.unparsed.contains("piecewise")
                                            || var.unparsed.contains("pw"))
                                    {
                                        parsed.1.push((
                                            var.name.iter().collect::<String>(),
                                            parsed.0.clone(),
                                        ))
                                    }
                                    vars[i].parsed = if var.name.contains(&'(')
                                    {
                                        parsed.0
                                    }
                                    else
                                    {
                                        vec![do_math(parsed.0, *options, parsed.1.clone())
                                            .unwrap_or(Num(Number::from(
                                                Complex::new(options.prec),
                                                None,
                                            )))]
                                    };
                                    vars[i].funcvars = parsed.1;
                                }
                            }
                        }
                    }
                    _ => return Err("Invalid precision"),
                },
                "range" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0].to_f64();
                        if range == 0.0
                        {
                            return Err("bad range");
                        }
                        (
                            options.xr.0,
                            options.xr.1,
                            options.yr.0,
                            options.yr.1,
                            options.zr.0,
                            options.zr.1,
                        ) = (-range, range, -range, range, -range, range)
                    }
                    else
                    {
                        let min = args[0].to_f64();
                        let max = args[1].to_f64();
                        if min == max
                        {
                            return Err("bad range");
                        }
                        (
                            options.xr.0,
                            options.xr.1,
                            options.yr.0,
                            options.yr.1,
                            options.zr.0,
                            options.zr.1,
                        ) = (min, max, min, max, min, max)
                    }
                }
                "xr" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0].to_f64();
                        if range == 0.0
                        {
                            return Err("bad range");
                        }
                        options.xr = (-range, range)
                    }
                    else
                    {
                        let min = args[0].to_f64();
                        let max = args[1].to_f64();
                        if min == max
                        {
                            return Err("bad range");
                        }
                        options.xr = (min, max)
                    }
                }
                "yr" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0].to_f64();
                        if range == 0.0
                        {
                            return Err("bad range");
                        }
                        options.yr = (-range, range)
                    }
                    else
                    {
                        let min = args[0].to_f64();
                        let max = args[1].to_f64();
                        if min == max
                        {
                            return Err("bad range");
                        }
                        options.yr = (min, max)
                    }
                }
                "zr" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0].to_f64();
                        if range == 0.0
                        {
                            return Err("bad range");
                        }
                        options.zr = (-range, range)
                    }
                    else
                    {
                        let min = args[0].to_f64();
                        let max = args[1].to_f64();
                        if min == max
                        {
                            return Err("bad range");
                        }
                        options.zr = (min, max)
                    }
                }
                "vrange" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0].to_f64();
                        if range == 0.0
                        {
                            return Err("bad range");
                        }
                        (
                            options.vxr.0,
                            options.vxr.1,
                            options.vyr.0,
                            options.vyr.1,
                            options.vzr.0,
                            options.vzr.1,
                        ) = (-range, range, -range, range, -range, range)
                    }
                    else
                    {
                        let min = args[0].to_f64();
                        let max = args[1].to_f64();
                        if min == max
                        {
                            return Err("bad range");
                        }
                        (
                            options.vxr.0,
                            options.vxr.1,
                            options.vyr.0,
                            options.vyr.1,
                            options.vzr.0,
                            options.vzr.1,
                        ) = (min, max, min, max, min, max)
                    }
                }
                "vxr" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0].to_f64();
                        if range == 0.0
                        {
                            return Err("bad range");
                        }
                        options.vxr = (-range, range)
                    }
                    else
                    {
                        let min = args[0].to_f64();
                        let max = args[1].to_f64();
                        if min == max
                        {
                            return Err("bad range");
                        }
                        options.vxr = (min, max)
                    }
                }
                "vyr" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0].to_f64();
                        if range == 0.0
                        {
                            return Err("bad range");
                        }
                        options.vyr = (-range, range)
                    }
                    else
                    {
                        let min = args[0].to_f64();
                        let max = args[1].to_f64();
                        if min == max
                        {
                            return Err("bad range");
                        }
                        options.vyr = (min, max)
                    }
                }
                "vzr" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0].to_f64();
                        if range == 0.0
                        {
                            return Err("bad range");
                        }
                        options.vzr = (-range, range)
                    }
                    else
                    {
                        let min = args[0].to_f64();
                        let max = args[1].to_f64();
                        if min == max
                        {
                            return Err("bad range");
                        }
                        options.vzr = (min, max)
                    }
                }
                "2d" =>
                {
                    if args[0].is_sign_negative()
                    {
                        options.samples_2d = (options.xr.1 - options.xr.0) as usize
                    }
                    else
                    {
                        options.samples_2d = args[0]
                            .to_integer()
                            .unwrap_or_default()
                            .to_usize()
                            .unwrap_or_default()
                    }
                }
                "3d" =>
                {
                    if args.len() == 1
                    {
                        if args[0].is_sign_negative()
                        {
                            options.samples_3d = (
                                (options.xr.1 - options.xr.0) as usize,
                                (options.yr.1 - options.yr.0) as usize,
                            )
                        }
                        else
                        {
                            let range = args[0]
                                .to_integer()
                                .unwrap_or_default()
                                .to_usize()
                                .unwrap_or_default();
                            options.samples_3d = (range, range)
                        }
                    }
                    else
                    {
                        if args[0].is_sign_negative()
                        {
                            options.samples_3d.0 = (options.xr.1 - options.xr.0) as usize
                        }
                        else
                        {
                            let range = args[0]
                                .to_integer()
                                .unwrap_or_default()
                                .to_usize()
                                .unwrap_or_default();
                            options.samples_3d.0 = range
                        }
                        if args[1].is_sign_negative()
                        {
                            options.samples_3d.1 = (options.yr.1 - options.yr.0) as usize
                        }
                        else
                        {
                            let range = args[1]
                                .to_integer()
                                .unwrap_or_default()
                                .to_usize()
                                .unwrap_or_default();
                            options.samples_3d.1 = range
                        }
                    }
                }
                _ => return Ok(()),
            }
        }
        _ => return Ok(()),
    }
    Err("")
}
pub fn silent_commands(options: &mut Options, input: &[char]) -> bool
{
    match input.iter().collect::<String>().as_str()
    {
        "default" | "defaults" | "reset" => *options = Options::default(),
        "interactive" => options.stay_interactive = !options.stay_interactive,
        "scalegraph" => options.scale_graph = !options.scale_graph,
        "debug" => options.debug = !options.debug,
        "color" | "colour" =>
        {
            options.color = match options.color
            {
                Auto::Auto | Auto::True => Auto::False,
                Auto::False => Auto::True,
            };
        }
        "prompt" => options.prompt = !options.prompt,
        "onaxis" => options.onaxis = !options.onaxis,
        "surface" => options.surface = !options.surface,
        "rt" => options.real_time_output = !options.real_time_output,
        "progress" => options.progress = !options.progress,
        "keep_data_file" => options.keep_data_file = !options.keep_data_file,
        "siunits" => options.si_units = !options.si_units,
        "keepzeros" => options.keep_zeros = !options.keep_zeros,
        "line" | "lines" =>
        {
            options.lines = match options.lines
            {
                Auto::Auto | Auto::False => Auto::True,
                Auto::True => Auto::False,
            };
        }
        "polar" => options.polar = !options.polar,
        "frac" | "fractions" => options.frac.num = !options.frac.num,
        "fractionsv" => options.frac.vec = !options.frac.vec,
        "fractionsm" => options.frac.mat = !options.frac.mat,
        "multi" => options.multi = !options.multi,
        "tabbed" => options.tabbed = !options.tabbed,
        "comma" => options.comma = !options.comma,
        "units" => options.units = !options.units,
        "vars" => options.allow_vars = !options.allow_vars,
        "graphcli" => options.graph_cli = !options.graph_cli,
        _ => return false,
    }
    true
}
#[allow(clippy::too_many_arguments)]
pub fn commands(options: &mut Options, lines: &[String], input: &[char], stdout: &mut Stdout)
{
    match input.iter().collect::<String>().trim_start().trim_end()
    {
        "graphcli" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.graph_cli = !options.graph_cli
        }
        "default" | "defaults" | "reset" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            *options = Options::default();
        }
        "scalegraph" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.scale_graph = !options.scale_graph
        }
        "color" | "colour" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.color = match options.color
            {
                Auto::Auto | Auto::True => Auto::False,
                Auto::False => Auto::True,
            };
        }
        "prompt" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.prompt = !options.prompt;
        }
        "onaxis" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.onaxis = !options.onaxis;
        }
        "surface" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.surface = !options.surface;
        }
        "rt" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.real_time_output = !options.real_time_output;
        }
        "progress" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.progress = !options.progress;
        }
        "siunits" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.si_units = !options.si_units;
        }
        "keepzeros" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.keep_zeros = !options.keep_zeros;
        }
        "clear" =>
        {
            execute!(stdout, Clear(ClearType::Purge)).unwrap();
            print!("\x1b[H\x1b[J");
            stdout.flush().unwrap();
        }
        "debug" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.debug = !options.debug;
        }
        "help" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            help();
        }
        "line" | "lines" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.lines = match options.lines
            {
                Auto::Auto | Auto::False => Auto::True,
                Auto::True => Auto::False,
            };
        }
        "polar" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.polar = !options.polar;
        }
        "frac" | "fractions" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.frac.num = !options.frac.num;
        }
        "fractionsv" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.frac.vec = !options.frac.vec;
        }
        "fractionsm" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.frac.mat = !options.frac.mat;
        }
        "multi" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.multi = !options.multi;
        }
        "tabbed" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.tabbed = !options.tabbed;
        }
        "comma" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.comma = !options.comma;
        }
        "units" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.units = !options.units;
        }
        "history" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            for l in lines
            {
                println!("{}\x1b[G", l);
            }
            stdout.flush().unwrap();
        }
        "exit" | "quit" | "break" =>
        {
            print!("\x1b[G\x1b[A\x1b[J");
            stdout.flush().unwrap();
            terminal::disable_raw_mode().unwrap();
            process::exit(0);
        }
        _ =>
        {
            let n = input.iter().collect::<String>();
            let mut split = n.splitn(2, ' ');
            match split.next().unwrap()
            {
                "history" | "his" =>
                {
                    print!("\x1b[A\x1b[G\x1b[K");
                    let r = split.next().unwrap();
                    for i in lines
                    {
                        if i.contains(r)
                        {
                            println!("{}\x1b[G", i);
                        }
                    }
                    stdout.flush().unwrap();
                }
                _ =>
                {}
            }
        }
    }
}
pub fn list_vars(vars: &[Variable], options: &Options, colors: &Colors) -> String
{
    let mut out = String::new();
    for v in vars.iter()
    {
        if !v.unparsed.is_empty()
        {
            out += format!(
                "{}={}\x1b[G\n",
                to_output(&v.name, vars, options.color == Auto::True, colors,),
                to_output(
                    &v.unparsed.chars().collect::<Vec<char>>(),
                    vars,
                    options.color == Auto::True,
                    colors,
                )
            )
            .as_str()
        }
        else
        {
            match &v.parsed[0]
            {
                Num(n) =>
                {
                    let n = custom_units(n.clone(), *options, colors);
                    let n = get_output(*options, colors, &n);
                    out += format!(
                        "{}={}{}{}{}\x1b[G\n",
                        v.name.iter().collect::<String>(),
                        n.0,
                        n.1,
                        n.2.unwrap_or_default(),
                        if options.color == Auto::True
                        {
                            &colors.text
                        }
                        else
                        {
                            ""
                        }
                    )
                    .as_str()
                }
                Vector(m) =>
                {
                    let mut st = String::new();
                    for i in m
                    {
                        let i = custom_units(i.clone(), *options, colors);
                        let n = get_output(*options, colors, &i);
                        st.push_str(&n.0);
                        st.push_str(&n.1);
                        st.push_str(&n.2.unwrap_or_default());
                        if options.color == Auto::True
                        {
                            st.push_str(&colors.text)
                        }
                        st.push(',');
                    }
                    out += format!(
                        "{}={{{}}}\x1b[G\n",
                        v.name.iter().collect::<String>(),
                        st.trim_end_matches(',')
                    )
                    .as_str()
                }
                Matrix(m) =>
                {
                    let mut st = String::new();
                    for i in m
                    {
                        st.push('{');
                        for g in i
                        {
                            let g = custom_units(g.clone(), *options, colors);
                            let n = get_output(*options, colors, &g);
                            st.push_str(&n.0);
                            st.push_str(&n.1);
                            st.push_str(&n.2.unwrap_or_default());
                            if options.color == Auto::True
                            {
                                st.push_str(&colors.text)
                            }
                            st.push(',');
                        }
                        st = st.trim_end_matches(',').to_string();
                        st.push('}');
                        st.push(',');
                    }
                    out += format!(
                        "{}={{{}}}\x1b[G\n",
                        v.name.iter().collect::<String>(),
                        st.trim_end_matches(',')
                    )
                    .as_str()
                }
                _ => continue,
            }
        }
    }
    out
}
pub fn equal_to(options: Options, colors: &Colors, vars: &[Variable], l: &str, last: &str)
    -> String
{
    match l.replace(' ', "").as_str()
    {
        "colors" =>
        {
            format!(
            "{}textc={} {}promptc={} {}imagc={} {}scic={} {}unitsc={} \x1b[0mbracketc={}\x1b[0m{}{}",
            colors.text,
            &colors.text[2..],
            colors.prompt,
            &colors.prompt[2..],
            colors.imag,
            &colors.imag[2..],
            colors.sci,
            &colors.sci[2..],
            colors.units,
            &colors.units[2..],
            bracketcol(&colors.brackets),
            colors.recol.iter().enumerate().map(|(i,a)| format!(" re{}col={}",i,formatcol(a))).collect::<Vec<String>>().concat(),
            colors.imcol.iter().enumerate().map(|(i,a)| format!(" im{}col={}",i,formatcol(a))).collect::<Vec<String>>().concat(),
        )
        }
        "slowcheck" => format!("{}", options.slowcheck),
        "label" => format!("{},{},{}", colors.label.0, colors.label.1, colors.label.2),
        "color" | "colour" => (match options.color
        {
            Auto::Auto => "auto",
            Auto::False => "false",
            Auto::True => "true",
        })
        .to_string(),
        "surface" => format!("{}", options.surface),
        "prompt" => format!("{}", options.prompt),
        "rt" => format!("{}", options.real_time_output),
        "progress" => format!("{}", options.progress),
        "siunits" => format!("{}", options.si_units),
        "keepzeros" => format!("{}", options.keep_zeros),
        "debug" => format!("{}", options.debug),
        "scalegraph" => format!("{}", options.scale_graph),
        "line" | "lines" => (match options.lines
        {
            Auto::Auto => "auto",
            Auto::False => "false",
            Auto::True => "true",
        })
        .to_string(),
        "polar" => format!("{}", options.polar),
        "frac" | "fractions" => format!("{}", options.frac.num),
        "fractionsv" => format!("{}", options.frac.vec),
        "fractionsm" => format!("{}", options.frac.mat),
        "multi" => format!("{}", options.multi),
        "tabbed" => format!("{}", options.tabbed),
        "comma" => format!("{}", options.comma),
        "units" => format!("{}", options.units),
        "graphcli" => format!("{}", options.graph_cli),
        "point" | "points" => format!("{}", options.point_style),
        "base" => format!("{} {}", options.base.0, options.base.1),
        "ticks" => format!(
            "x:{} y:{} z:{}",
            options.ticks.0, options.ticks.1, options.ticks.2
        ),
        "onaxis" => format!("{}", options.onaxis),
        "decimal" | "deci" | "decimals" => format!("{}", options.decimal_places),
        "prec" | "precision" => format!("{}", options.prec),
        "windowsize" => format!("{},{}", options.window_size.0, options.window_size.1),
        "graphprec" | "graphprecision" => format!("{}", options.graph_prec),
        "xr" => format!("{},{}", options.xr.0, options.xr.1),
        "yr" => format!("{},{}", options.yr.0, options.yr.1),
        "zr" => format!("{},{}", options.zr.0, options.zr.1),
        "range" => format!(
            "x:{},{} y:{},{} z:{},{}",
            options.xr.0, options.xr.1, options.yr.0, options.yr.1, options.zr.0, options.zr.1
        ),
        "vxr" => format!("{},{}", options.vxr.0, options.vxr.1),
        "vyr" => format!("{},{}", options.vyr.0, options.vyr.1),
        "vzr" => format!("{},{}", options.vzr.0, options.vzr.1),
        "vrange" => format!(
            "x:{},{} y:{},{} z:{},{}",
            options.vxr.0,
            options.vxr.1,
            options.vyr.0,
            options.vyr.1,
            options.vzr.0,
            options.vzr.1
        ),
        "2d" => format!("{}", options.samples_2d),
        "3d" => format!("{} {}", options.samples_3d.0, options.samples_3d.1),
        "angle" => match options.angle
        {
            Degrees => "deg",
            Radians => "rad",
            Gradians => "grad",
        }
        .to_string(),
        "notation" => match options.notation
        {
            SmallEngineering => "e",
            LargeEngineering => "E",
            Scientific => "s",
            Normal => "n",
        }
        .to_string(),
        "graph" => match options.graphtype
        {
            GraphType::Normal => "normal",
            GraphType::Domain => "domain",
            GraphType::DomainAlt => "domain_alt",
            GraphType::None => "none",
            GraphType::Depth => "depth",
            GraphType::Flat => "flat",
        }
        .to_string(),
        "interactive" => format!("{}", options.interactive),
        "textc" => colors.text.to_string(),
        "promptc" => colors.prompt.to_string(),
        "imagc" => colors.imag.to_string(),
        "scic" => colors.sci.to_string(),
        "unitsc" => colors.units.to_string(),
        "bracketc" => bracketcol(&colors.brackets),
        "default_units" => colors
            .default_units
            .iter()
            .map(|a| a.0.clone())
            .collect::<Vec<String>>()
            .join(","),
        "saveto" => colors.graphtofile.to_string(),
        "recol" => colors
            .recol
            .iter()
            .enumerate()
            .map(|(i, a)| format!("re{}col={}", i, formatcol(a)))
            .collect::<Vec<String>>()
            .join(" "),
        "imcol" => colors
            .imcol
            .iter()
            .enumerate()
            .map(|(i, a)| format!("im{}col={}", i, formatcol(a)))
            .collect::<Vec<String>>()
            .join(" "),
        _ =>
        {
            let input = input_var(
                &insert_last(&l.chars().collect::<Vec<char>>(), last),
                vars,
                &mut Vec::new(),
                &mut 0,
                options,
                true,
                0,
                Vec::new(),
                false,
                &mut Vec::new(),
                None,
            );
            match input
            {
                Ok(f) => parsed_to_string(f.0, vars, f.1, &options, colors),
                Err(s) => s.to_string(),
            }
        }
    }
}
fn formatcol(color: &str) -> String
{
    format!(
        "\x1b[38;2;{};{};{}m{}\x1b[0m",
        u8::from_str_radix(&color[1..3], 16).unwrap(),
        u8::from_str_radix(&color[3..5], 16).unwrap(),
        u8::from_str_radix(&color[5..7], 16).unwrap(),
        color
    )
}
fn bracketcol(bracket: &[String]) -> String
{
    bracket
        .iter()
        .fold(String::new(), |out, a| out + a + &a[2..] + ",")
        .trim_end_matches(',')
        .to_string()
}
