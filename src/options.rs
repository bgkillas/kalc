use crate::{
    complex::NumStr::{Matrix, Num, Str, Vector},
    help::help,
    math::do_math,
    misc::{prompt, to_output},
    options::AngleType::{Degrees, Gradians, Radians},
    parse::get_func,
    print::get_output,
    vars::{get_vars, input_var},
    Colors, Options,
};
use crossterm::terminal;
use std::{
    fs::File,
    io::{BufRead, BufReader, Stdout, Write},
    time::Instant,
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
            "--debug" => options.debug = !options.debug,
            "--depth" => options.depth = !options.depth,
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
                        Ok(x) if x != 0 => x,
                        _ => return Err("invalid prec"),
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
                        options.decimal_places =
                            args[i + 1].parse::<usize>().expect("invalid deci");
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
                    colors.text =
                        "\x1b[".to_owned() + &args[i + 1].parse::<String>().expect("invalid col");
                    args.remove(i);
                }
            }
            "--promptc" =>
            {
                if args.len() > 1
                {
                    colors.prompt =
                        "\x1b[".to_owned() + &args[i + 1].parse::<String>().expect("invalid col");
                    args.remove(i);
                }
            }
            "--imagc" =>
            {
                if args.len() > 1
                {
                    colors.imag =
                        "\x1b[".to_owned() + &args[i + 1].parse::<String>().expect("invalid col");
                    args.remove(i);
                }
            }
            "--scic" =>
            {
                if args.len() > 1
                {
                    colors.sci =
                        "\x1b[".to_owned() + &args[i + 1].parse::<String>().expect("invalid col");
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
            "--comma" => options.comma = !options.comma,
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
            "--default" | "--def" =>
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
                "frac_iter" =>
                {
                    options.frac_iter = split
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .expect("invalid frac_iter")
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
                    let mut xr = split.next().unwrap().split(',');
                    if xr.clone().count() != 2
                    {
                        return Err("invalid x range");
                    }
                    options.xr.0 = xr.next().unwrap().parse::<f64>().expect("invalid xr");
                    options.xr.1 = xr.next().unwrap().parse::<f64>().expect("invalid xr")
                }
                "yr" =>
                {
                    let mut yr = split.next().unwrap().split(',');
                    if yr.clone().count() != 2
                    {
                        return Err("invalid y range");
                    }
                    options.yr.0 = yr.next().unwrap().parse::<f64>().expect("invalid yr");
                    options.yr.1 = yr.next().unwrap().parse::<f64>().expect("invalid yr")
                }
                "zr" =>
                {
                    let mut zr = split.next().unwrap().split(',');
                    if zr.clone().count() != 2
                    {
                        return Err("invalid z range");
                    }
                    options.zr.0 = zr.next().unwrap().parse::<f64>().expect("invalid zr");
                    options.zr.1 = zr.next().unwrap().parse::<f64>().expect("invalid zr")
                }
                "prec" | "precision" =>
                {
                    options.prec = match split.next().unwrap().parse::<u32>()
                    {
                        Ok(x) if x != 0 => x,
                        _ => return Err("invalid prec"),
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
                    options.polar = split.next().unwrap().parse::<bool>().expect("invalid frac")
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
                "debug" =>
                {
                    options.debug = split
                        .next()
                        .unwrap()
                        .parse::<bool>()
                        .expect("invalid debug")
                }
                "deg" => options.deg = Radians,
                "rad" => options.deg = Degrees,
                "grad" => options.deg = Gradians,
                "tau" => options.tau = split.next().unwrap().parse::<bool>().expect("invalid tau"),
                "vars" =>
                {
                    options.allow_vars = split.next().unwrap().parse::<bool>().expect("invalid var")
                }
                _ =>
                {}
            }
        }
    }
    Ok(())
}
pub fn equal_to(
    options: Options,
    colors: &Colors,
    vars: &[[String; 2]],
    l: &str,
    last: &str,
) -> String
{
    match l
    {
        "colors" => format!(
            "{}textc={} {}promptc={} {}imagc={} {}scic={} \x1b[0mbracketc={}\x1b[0m",
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
                .fold(String::new(), |out, a| out + &format!("{}{},", a, &a[2..]))
        ),
        "color" => format!("{}", options.color),
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
        "point" => format!("{}", options.point_style),
        "base" => format!("{}", options.base),
        "decimal" | "deci" | "decimals" => format!("{}", options.decimal_places),
        "prec" | "precision" => format!("{}", options.prec),
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
            let mut out = String::new();
            for i in match get_func(
                &input_var(
                    &l.replace('_', &format!("({})", last)),
                    vars,
                    &Vec::new(),
                    None,
                    &mut Vec::new(),
                    &mut 0,
                    options,
                ),
                options,
            )
            {
                Ok(n) => n,
                _ => return "".to_string(),
            }
            {
                match i
                {
                    Num(n) =>
                    {
                        let n = get_output(options, colors, &n);
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
                            num = get_output(options, colors, &i);
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
                                num = get_output(options, colors, &j);
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
                    Str(n) => out.push_str(&n),
                }
            }
            to_output(&out.chars().collect::<Vec<char>>(), options.color, colors)
        }
    }
}
pub fn set_commands(
    options: &mut Options,
    colors: &mut Colors,
    vars: &mut [[String; 2]],
    old: &mut Vec<[String; 2]>,
    l: &str,
    r: &str,
) -> Result<(), &'static str>
{
    match l
    {
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
        "decimal" | "deci" | "decimals" =>
        {
            match do_math(
                get_func(
                    &input_var(
                        r,
                        vars,
                        &Vec::new(),
                        None,
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                    ),
                    *options,
                )?,
                *options,
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
        "prec" | "precision" => match do_math(
            get_func(
                &input_var(
                    r,
                    vars,
                    &Vec::new(),
                    None,
                    &mut Vec::new(),
                    &mut 0,
                    *options,
                ),
                *options,
            )?,
            *options,
        )?
        .num()?
        .real()
        .to_f64() as u32
        {
            n if n != 0 =>
            {
                options.prec = n;
                if options.allow_vars
                {
                    let v = get_vars(*options);
                    for i in old.clone()
                    {
                        for (j, var) in vars.iter_mut().enumerate()
                        {
                            if v.len() > j && i[0] == v[j][0] && i[1] == var[1]
                            {
                                *var = v[j].clone();
                            }
                        }
                    }
                    *old = v;
                }
            }
            _ => return Err("Invalid precision"),
        },
        "range" =>
        {
            if r.contains(',')
            {
                let (min, max) = (
                    do_math(
                        get_func(
                            &input_var(
                                r.split(',').next().unwrap(),
                                vars,
                                &Vec::new(),
                                None,
                                &mut Vec::new(),
                                &mut 0,
                                *options,
                            ),
                            *options,
                        )?,
                        *options,
                    )?
                    .num()?
                    .real()
                    .to_f64(),
                    do_math(
                        get_func(
                            &input_var(
                                r.split(',').last().unwrap(),
                                vars,
                                &Vec::new(),
                                None,
                                &mut Vec::new(),
                                &mut 0,
                                *options,
                            ),
                            *options,
                        )?,
                        *options,
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
                    get_func(
                        &input_var(
                            r,
                            vars,
                            &Vec::new(),
                            None,
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                        ),
                        *options,
                    )?,
                    *options,
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
                options.xr.0 = do_math(
                    get_func(
                        &input_var(
                            r.split(',').next().unwrap(),
                            vars,
                            &Vec::new(),
                            None,
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                        ),
                        *options,
                    )?,
                    *options,
                )?
                .num()?
                .real()
                .to_f64();
                options.xr.1 = do_math(
                    get_func(
                        &input_var(
                            r.split(',').last().unwrap(),
                            vars,
                            &Vec::new(),
                            None,
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                        ),
                        *options,
                    )?,
                    *options,
                )?
                .num()?
                .real()
                .to_f64();
            }
            else
            {
                let n = do_math(
                    get_func(
                        &input_var(
                            r,
                            vars,
                            &Vec::new(),
                            None,
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                        ),
                        *options,
                    )?,
                    *options,
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
                options.yr.0 = do_math(
                    get_func(
                        &input_var(
                            r.split(',').next().unwrap(),
                            vars,
                            &Vec::new(),
                            None,
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                        ),
                        *options,
                    )?,
                    *options,
                )?
                .num()?
                .real()
                .to_f64();
                options.yr.1 = do_math(
                    get_func(
                        &input_var(
                            r.split(',').last().unwrap(),
                            vars,
                            &Vec::new(),
                            None,
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                        ),
                        *options,
                    )?,
                    *options,
                )?
                .num()?
                .real()
                .to_f64();
            }
            else
            {
                let n = do_math(
                    get_func(
                        &input_var(
                            r,
                            vars,
                            &Vec::new(),
                            None,
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                        ),
                        *options,
                    )?,
                    *options,
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
                options.zr.0 = do_math(
                    get_func(
                        &input_var(
                            r.split(',').next().unwrap(),
                            vars,
                            &Vec::new(),
                            None,
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                        ),
                        *options,
                    )?,
                    *options,
                )?
                .num()?
                .real()
                .to_f64();
                options.zr.1 = do_math(
                    get_func(
                        &input_var(
                            r.split(',').last().unwrap(),
                            vars,
                            &Vec::new(),
                            None,
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                        ),
                        *options,
                    )?,
                    *options,
                )?
                .num()?
                .real()
                .to_f64();
            }
            else
            {
                let n = do_math(
                    get_func(
                        &input_var(
                            r,
                            vars,
                            &Vec::new(),
                            None,
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                        ),
                        *options,
                    )?,
                    *options,
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
                get_func(
                    &input_var(
                        r,
                        vars,
                        &Vec::new(),
                        None,
                        &mut Vec::new(),
                        &mut 0,
                        *options,
                    ),
                    *options,
                )?,
                *options,
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
                        get_func(
                            &input_var(
                                r.split(',').next().unwrap(),
                                vars,
                                &Vec::new(),
                                None,
                                &mut Vec::new(),
                                &mut 0,
                                *options,
                            ),
                            *options,
                        )?,
                        *options,
                    )?
                    .num()?
                    .real()
                    .to_f64() as usize,
                    do_math(
                        get_func(
                            &input_var(
                                r.split(',').last().unwrap(),
                                vars,
                                &Vec::new(),
                                None,
                                &mut Vec::new(),
                                &mut 0,
                                *options,
                            ),
                            *options,
                        )?,
                        *options,
                    )?
                    .num()?
                    .real()
                    .to_f64() as usize,
                );
            }
            else
            {
                let n = do_math(
                    get_func(
                        &input_var(
                            r,
                            vars,
                            &Vec::new(),
                            None,
                            &mut Vec::new(),
                            &mut 0,
                            *options,
                        ),
                        *options,
                    )?,
                    *options,
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
#[allow(clippy::too_many_arguments)]
pub fn commands(
    options: &mut Options,
    colors: &Colors,
    watch: &mut Option<Instant>,
    vars: &mut [[String; 2]],
    old: &mut Vec<[String; 2]>,
    lines: &[String],
    input: &[char],
    stdout: &mut Stdout,
)
{
    match input.iter().collect::<String>().as_str()
    {
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
            if options.allow_vars
            {
                let v = get_vars(*options);
                for i in old.clone()
                {
                    for (j, var) in vars.iter_mut().enumerate()
                    {
                        if v.len() > j && i[0] == v[j][0] && i[1] == var[1]
                        {
                            *var = v[j].clone();
                        }
                    }
                }
                *old = v;
            }
        }
        "sci" | "scientific" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.sci = !options.sci;
        }
        "clear" =>
        {
            print!("\x1b[H\x1b[J{}", prompt(*options, colors));
            stdout.flush().unwrap();
        }
        "debug" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            options.debug = !options.debug;
            *watch = None;
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
                if v[0].contains('(')
                {
                    println!("{}={}", v[0], v[1]);
                }
                else
                {
                    match &do_math(
                        get_func(
                            &input_var(
                                &v[1],
                                vars,
                                &Vec::new(),
                                None,
                                &mut Vec::new(),
                                &mut 0,
                                *options,
                            ),
                            *options,
                        )
                        .unwrap(),
                        *options,
                    )
                    .unwrap()
                    {
                        Num(n) =>
                        {
                            let n = get_output(*options, colors, n);
                            print!("{}={}{}\n\x1b[G", v[0], n.0, n.1)
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
                            print!("{}={{{}}}\n\x1b[G", v[0], st.trim_end_matches(','))
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
                            print!("{}={{{}}}\n\x1b[G", v[0], st.trim_end_matches(','))
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
                print!("{}={}\n\x1b[G", v[0], v[1]);
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
#[derive(Copy, Clone, PartialEq)]
pub enum AngleType
{
    Radians,
    Degrees,
    Gradians,
}