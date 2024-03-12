use crate::{
    complex::NumStr::{Matrix, Num, Vector},
    help::help,
    load_vars::get_vars,
    math::do_math,
    misc::{parsed_to_string, to_output},
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
    io::{BufRead, BufReader, Stdout, Write},
    process,
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
        match args[i].as_str()
        {
            "-i" =>
            {
                options.stay_interactive = !options.stay_interactive;
                args.remove(i);
            }
            "-v" =>
            {
                println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                process::exit(0)
            }
            "-h" =>
            {
                help();
                process::exit(0)
            }
            "--" =>
            {
                args.remove(i);
                break;
            }
            _ if !args[i].starts_with('-') =>
            {
                break;
            }
            _ =>
            {
                let arg = args[i].trim_start_matches('-');
                let mut split = arg.splitn(2, '=');
                if split.clone().count() == 2
                {
                    match set_commands(
                        options,
                        colors,
                        &mut Vec::new(),
                        split.next().unwrap(),
                        split.next().unwrap(),
                    )
                    {
                        Err(s) if !s.is_empty() => return Err(s),
                        _ =>
                        {}
                    }
                }
                else
                {
                    silent_commands(options, &arg.chars().collect::<Vec<char>>())
                }
                args.remove(i);
                i += 1;
            }
        }
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
        for line in reader.lines().map(|l| l.unwrap())
        {
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
                    &mut Vec::new(),
                    split.next().unwrap(),
                    split.next().unwrap(),
                )
                {
                    Err(s) if !s.is_empty() => return Err(s),
                    _ =>
                    {}
                }
            }
            else
            {
                silent_commands(options, &line.chars().collect::<Vec<char>>())
            }
        }
    }
    Ok(())
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
        "textc" => colors.text = "\x1b[".to_owned() + r,
        "promptc" => colors.prompt = "\x1b[".to_owned() + r,
        "imagc" => colors.imag = "\x1b[".to_owned() + r,
        "scic" => colors.sci = "\x1b[".to_owned() + r,
        "bracketc" =>
        {
            colors.brackets = r
                .split(',')
                .map(|a| "\x1b[".to_owned() + a)
                .collect::<Vec<String>>()
        }
        "label" =>
        {
            let mut split = r.split(',');
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
        "interactive" | "color" | "prompt" | "depth" | "surface" | "flat" | "rt" | "small_e"
        | "sci" | "scientific" | "line" | "lines" | "polar" | "frac" | "multi" | "tabbed"
        | "comma" | "graph" | "var_multiply" | "scalegraph" | "debug" | "vars" | "onaxis"
        | "base" | "ticks" | "decimal" | "deci" | "decimals" | "graphprec" | "graphprecision"
        | "prec" | "precision" | "range" | "xr" | "yr" | "zr" | "vrange" | "vxr" | "vyr"
        | "vzr" | "frac_iter" | "2d" | "3d" =>
        {
            let mut args: Vec<f64> = Vec::new();
            {
                let mut bracket = 0;
                let mut last = 0;
                for (i, c) in r.chars().enumerate()
                {
                    match c
                    {
                        '(' | '{' | '[' => bracket += 1,
                        ')' | '}' | ']' => bracket -= 1,
                        ',' if bracket == 0 =>
                        {
                            let parsed = input_var(
                                &r[last..i],
                                vars.to_vec(),
                                &mut Vec::new(),
                                &mut 0,
                                *options,
                                false,
                                false,
                                0,
                                Vec::new(),
                            )?;
                            args.push(
                                do_math(parsed.0, *options, parsed.1)?
                                    .num()?
                                    .real()
                                    .to_f64(),
                            );
                            last = i + 1;
                        }
                        _ =>
                        {}
                    }
                }
                let parsed = input_var(
                    &r[last..],
                    vars.to_vec(),
                    &mut Vec::new(),
                    &mut 0,
                    *options,
                    false,
                    false,
                    0,
                    Vec::new(),
                )?;
                args.push(
                    do_math(parsed.0, *options, parsed.1)?
                        .num()?
                        .real()
                        .to_f64(),
                );
            }
            match l
            {
                "color" => options.color = args[0] != 0.0,
                "interactive" => options.stay_interactive = args[0] != 0.0,
                "prompt" => options.prompt = args[0] != 0.0,
                "depth" => options.depth = args[0] != 0.0,
                "surface" => options.surface = args[0] != 0.0,
                "flat" => options.flat = args[0] != 0.0,
                "rt" => options.real_time_output = args[0] != 0.0,
                "small_e" => options.small_e = args[0] != 0.0,
                "sci" | "scientific" => options.sci = args[0] != 0.0,
                "line" | "lines" => options.lines = args[0] != 0.0,
                "polar" => options.polar = args[0] != 0.0,
                "frac" => options.frac = args[0] != 0.0,
                "multi" => options.multi = args[0] != 0.0,
                "tabbed" => options.tabbed = args[0] != 0.0,
                "comma" => options.comma = args[0] != 0.0,
                "graph" => options.graph = args[0] != 0.0,
                "var_multiply" => options.var_multiply = args[0] != 0.0,
                "scalegraph" => options.scale_graph = args[0] != 0.0,
                "debug" => options.debug = args[0] != 0.0,
                "vars" => options.allow_vars = args[0] != 0.0,
                "onaxis" => options.onaxis = args[0] != 0.0,
                "base" =>
                {
                    let n = args[0] as usize;
                    if (2..=36).contains(&n)
                    {
                        options.base = n
                    }
                    else
                    {
                        return Err("out of range of 2..=36");
                    }
                }
                "ticks" => options.ticks = args[0],
                "decimal" | "deci" | "decimals" =>
                {
                    options.decimal_places = match args[0] as isize
                    {
                        -1 => usize::MAX - 1,
                        -2 => usize::MAX,
                        n if n >= 0 => n as usize,
                        _ => return Err("Invalid decimal"),
                    };
                }
                "graphprec" | "graphprecision" => match args[0] as u32
                {
                    n if n != 0 => options.graph_prec = (n, n),
                    _ => return Err("Invalid graphprecision"),
                },
                "prec" | "precision" => match args[0] as u32
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
                                                    vars.to_vec(),
                                                    &mut func_vars,
                                                    &mut 0,
                                                    *options,
                                                    false,
                                                    false,
                                                    0,
                                                    s.chars().collect::<Vec<char>>(),
                                                )?;
                                                func_vars.push((-1, s.clone()));
                                                fvs.push((s, parsed.0));
                                                fvs.extend(parsed.1)
                                            }
                                        }
                                    }
                                    let mut parsed = match input_var(
                                        &unparsed,
                                        vars.to_vec(),
                                        &mut func_vars,
                                        &mut 0,
                                        *options,
                                        false,
                                        false,
                                        0,
                                        var.name.clone(),
                                    )
                                    {
                                        Ok(n) => (n.0, n.1),
                                        _ => return Err("prec crash"),
                                    };
                                    parsed.1.extend(fvs);
                                    vars[i].parsed = if var.name.contains(&'(')
                                    {
                                        parsed.0
                                    }
                                    else
                                    {
                                        vec![do_math(parsed.0, *options, parsed.1.clone())
                                            .unwrap_or(Num(Complex::new(options.prec)))]
                                    };
                                    vars[i].funcvars = parsed.1;
                                    if var.name.contains(&'(')
                                        && var
                                            .unparsed
                                            .contains(var.name.split(|c| c == &'(').next().unwrap())
                                        && (var.unparsed.contains("piecewise")
                                            || var.unparsed.contains("pw"))
                                    {
                                        let parsed = vars[i].parsed.clone();
                                        vars[i]
                                            .funcvars
                                            .push((var.name.iter().collect::<String>(), parsed))
                                    }
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
                        let range = args[0];
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
                        let min = args[0];
                        let max = args[1];
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
                        let range = args[0];
                        options.xr = (-range, range)
                    }
                    else
                    {
                        let min = args[0];
                        let max = args[1];
                        options.xr = (min, max)
                    }
                }
                "yr" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0];
                        options.yr = (-range, range)
                    }
                    else
                    {
                        let min = args[0];
                        let max = args[1];
                        options.yr = (min, max)
                    }
                }
                "zr" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0];
                        options.zr = (-range, range)
                    }
                    else
                    {
                        let min = args[0];
                        let max = args[1];
                        options.zr = (min, max)
                    }
                }
                "vrange" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0];
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
                        let min = args[0];
                        let max = args[1];
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
                        let range = args[0];
                        options.vxr = (-range, range)
                    }
                    else
                    {
                        let min = args[0];
                        let max = args[1];
                        options.vxr = (min, max)
                    }
                }
                "vyr" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0];
                        options.vyr = (-range, range)
                    }
                    else
                    {
                        let min = args[0];
                        let max = args[1];
                        options.vyr = (min, max)
                    }
                }
                "vzr" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0];
                        options.vzr = (-range, range)
                    }
                    else
                    {
                        let min = args[0];
                        let max = args[1];
                        options.vzr = (min, max)
                    }
                }
                "frac_iter" => options.frac_iter = args[0] as usize,
                "2d" => options.samples_2d = args[0] as usize,
                "3d" =>
                {
                    if args.len() == 1
                    {
                        let range = args[0] as usize;
                        options.samples_3d = (range, range)
                    }
                    else
                    {
                        let x = args[0] as usize;
                        let y = args[1] as usize;
                        options.samples_3d = (x, y)
                    }
                }
                _ => return Ok(()),
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
        "var_multiply" => options.var_multiply = !options.var_multiply,
        "interactive" => options.stay_interactive = !options.stay_interactive,
        "scalegraph" => options.scale_graph = !options.scale_graph,
        "debug" => options.debug = !options.debug,
        "color" => options.color = !options.color,
        "prompt" => options.prompt = !options.prompt,
        "depth" => options.depth = !options.depth,
        "onaxis" => options.onaxis = !options.onaxis,
        "surface" => options.surface = !options.surface,
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
        "vars" => options.allow_vars = !options.allow_vars,
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
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            *options = Options::default();
        }
        "var_multiply" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.var_multiply = !options.var_multiply
        }
        "scalegraph" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.scale_graph = !options.scale_graph
        }
        "color" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.color = !options.color;
        }
        "prompt" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.prompt = !options.prompt;
        }
        "depth" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.depth = !options.depth;
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
        "flat" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.flat = !options.flat;
        }
        "deg" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.deg = Degrees;
        }
        "rad" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.deg = Radians;
        }
        "grad" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.deg = Gradians;
        }
        "rt" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.real_time_output = !options.real_time_output;
        }
        "tau" => options.tau = true,
        "pi" => options.tau = false,
        "small_e" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.small_e = !options.small_e;
        }
        "sci" | "scientific" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.sci = !options.sci;
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
            options.lines = !options.lines;
        }
        "polar" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.polar = !options.polar;
        }
        "frac" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.frac = !options.frac;
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
        "graph" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            stdout.flush().unwrap();
            options.graph = !options.graph;
        }
        "history" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            for l in lines
            {
                print!("{}\x1b[G\n", l);
            }
            stdout.flush().unwrap();
        }
        "vars" | "variables" =>
        {
            print!("\x1b[G\x1b[A\x1b[K");
            for v in vars.iter()
            {
                if !v.unparsed.is_empty()
                {
                    print!(
                        "{}={}\x1b[G\n",
                        v.name.iter().collect::<String>(),
                        to_output(
                            &v.unparsed.chars().collect::<Vec<char>>(),
                            options.color,
                            colors,
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
                                "{}={}{}{}\x1b[G\n",
                                v.name.iter().collect::<String>(),
                                n.0,
                                n.1,
                                if options.color { &colors.text } else { "" }
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
                                if options.color
                                {
                                    st.push_str(&colors.text)
                                }
                                st.push(',');
                            }
                            print!(
                                "{}={{{}}}\x1b[G\n",
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
                                    if options.color
                                    {
                                        st.push_str(&colors.text)
                                    }
                                    st.push(',');
                                }
                                st = st.trim_end_matches(',').to_string();
                                st.push('}');
                                st.push(',');
                            }
                            print!(
                                "{}={{{}}}\x1b[G\n",
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
        "version" => println!(
            "\x1b[G\x1b[A\x1b[K{} {}\x1b[G",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ),
        "exit" | "quit" | "break" =>
        {
            print!("\x1b[G\x1b[A\x1b[J");
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
                print!("\x1b[G\x1b[A\x1b[K");
                let r = split.next().unwrap();
                for i in lines
                {
                    if i.contains(r)
                    {
                        print!("{}\x1b[G\n", i);
                    }
                }
                stdout.flush().unwrap();
            }
        }
    }
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
            u8::from_str_radix(&colors.re1col[1..3], 16).unwrap(),
            u8::from_str_radix(&colors.re1col[3..5], 16).unwrap(),
            u8::from_str_radix(&colors.re1col[5..7], 16).unwrap(),
            colors.re1col,
            u8::from_str_radix(&colors.im1col[1..3], 16).unwrap(),
            u8::from_str_radix(&colors.im1col[3..5], 16).unwrap(),
            u8::from_str_radix(&colors.im1col[5..7], 16).unwrap(),
            colors.im1col,
            u8::from_str_radix(&colors.re2col[1..3], 16).unwrap(),
            u8::from_str_radix(&colors.re2col[3..5], 16).unwrap(),
            u8::from_str_radix(&colors.re2col[5..7], 16).unwrap(),
            colors.re2col,
            u8::from_str_radix(&colors.im2col[1..3], 16).unwrap(),
            u8::from_str_radix(&colors.im2col[3..5], 16).unwrap(),
            u8::from_str_radix(&colors.im2col[5..7], 16).unwrap(),
            colors.im2col,
            u8::from_str_radix(&colors.re3col[1..3], 16).unwrap(),
            u8::from_str_radix(&colors.re3col[3..5], 16).unwrap(),
            u8::from_str_radix(&colors.re3col[5..7], 16).unwrap(),
            colors.re3col,
            u8::from_str_radix(&colors.im3col[1..3], 16).unwrap(),
            u8::from_str_radix(&colors.im3col[3..5], 16).unwrap(),
            u8::from_str_radix(&colors.im3col[5..7], 16).unwrap(),
            colors.im3col,
            u8::from_str_radix(&colors.re4col[1..3], 16).unwrap(),
            u8::from_str_radix(&colors.re4col[3..5], 16).unwrap(),
            u8::from_str_radix(&colors.re4col[5..7], 16).unwrap(),
            colors.re4col,
            u8::from_str_radix(&colors.im4col[1..3], 16).unwrap(),
            u8::from_str_radix(&colors.im4col[3..5], 16).unwrap(),
            u8::from_str_radix(&colors.im4col[5..7], 16).unwrap(),
            colors.im4col,
            u8::from_str_radix(&colors.re5col[1..3], 16).unwrap(),
            u8::from_str_radix(&colors.re5col[3..5], 16).unwrap(),
            u8::from_str_radix(&colors.re5col[5..7], 16).unwrap(),
            colors.re5col,
            u8::from_str_radix(&colors.im5col[1..3], 16).unwrap(),
            u8::from_str_radix(&colors.im5col[3..5], 16).unwrap(),
            u8::from_str_radix(&colors.im5col[5..7], 16).unwrap(),
            colors.im5col,
            u8::from_str_radix(&colors.re6col[1..3], 16).unwrap(),
            u8::from_str_radix(&colors.re6col[3..5], 16).unwrap(),
            u8::from_str_radix(&colors.re6col[5..7], 16).unwrap(),
            colors.re6col,
            u8::from_str_radix(&colors.im6col[1..3], 16).unwrap(),
            u8::from_str_radix(&colors.im6col[3..5], 16).unwrap(),
            u8::from_str_radix(&colors.im6col[5..7], 16).unwrap(),
            colors.im6col,
        ),
        "label"=>format!("{},{},{}",colors.label.0,colors.label.1,colors.label.2),
        "color" => format!("{}", options.color),
        "depth" => format!("{}", options.depth),
        "surface" => format!("{}", options.surface),
        "flat" => format!("{}", options.flat),
        "prompt" => format!("{}", options.prompt),
        "rt" => format!("{}", options.real_time_output),
        "sci" | "scientific" => format!("{}", options.sci),
        "debug" => format!("{}", options.debug),
        "scalegraph" => format!("{}", options.scale_graph),
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
        "onaxis" => format!("{}", options.onaxis),
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
        "vxr" => format!("{},{}", options.vxr.0, options.vxr.1),
        "vyr" => format!("{},{}", options.vyr.0, options.vyr.1),
        "vzr" => format!("{},{}", options.vzr.0, options.vzr.1),
        "vrange" => format!(
            "x:{},{} y:{},{} z:{},{}",
            options.vxr.0, options.vxr.1, options.vyr.0, options.vyr.1, options.vzr.0, options.vzr.1
        ),
        "frac_iter" => format!("{}", options.frac_iter),
        "2d" => format!("{}", options.samples_2d),
        "3d" => format!("{} {}", options.samples_3d.0, options.samples_3d.1),
        _ =>
            {
                let input = input_var(
                    &l.replace('_', &format!("({})", last)),
                    vars.to_vec(),
                    &mut Vec::new(),
                    &mut 0,
                    options, false,
                    true,
                    0,
                    Vec::new(),
                );
                if let Ok(f) = input
                {
                    parsed_to_string(f.0, f.1, &options, colors)
                } else {
                    String::new()
                }
            }
    }
}
