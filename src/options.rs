use crate::{
    complex::NumStr::{Matrix, Num, Str, Vector},
    help::help,
    math::do_math,
    options::AngleType::{Degrees, Gradians, Radians},
    parse::get_func,
    print::get_output,
    vars::{get_vars, input_var},
    Options,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Stdout, Write},
    time::Instant,
};
pub fn arg_opts(options: &mut Options, args: &mut Vec<String>) -> bool
{
    let mut err = false;
    args.remove(0);
    let (mut split, mut l);
    let mut i = 0;
    while i < args.len()
    {
        if args[i].starts_with("--") && (args[i].contains('=') || args[i].contains(','))
        {
            l = args[i].clone();
            split = l.split(|c| c == '=' || c == ',');
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
                        Ok(x) =>
                        {
                            if x == 0
                            {
                                println!("Invalid precision");
                                err = true;
                                args.remove(i);
                                continue;
                            }
                            else
                            {
                                x
                            }
                        }
                        Err(_) =>
                        {
                            println!("Invalid precision");
                            err = true;
                            args.remove(i);
                            continue;
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
                            Ok(x) => x,
                            Err(_) =>
                            {
                                println!("Invalid decimal");
                                err = true;
                                args.remove(i);
                                continue;
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
                    options.frac_iter = match args[i + 1].parse::<usize>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid frac iter");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    args.remove(i);
                }
            }
            "--2d" =>
            {
                if args.len() > 1
                {
                    options.samples_2d = match args[i + 1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid 2d sample size");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    args.remove(i);
                }
            }
            "--3d" =>
            {
                if args.len() > 2
                {
                    options.samples_3d.0 = match args[i + 1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid 3d sample size");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    options.samples_3d.1 = match args[i + 2].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid 3d sample size");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    args.remove(i);
                    args.remove(i);
                }
            }
            "--yr" =>
            {
                if args.len() > 2
                {
                    options.yr.0 = match args[i + 1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    options.yr.1 = match args[i + 2].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    args.remove(i);
                    args.remove(i);
                }
            }
            "--range" =>
            {
                if args.len() > 1
                {
                    (
                        options.xr.0,
                        options.xr.1,
                        options.yr.0,
                        options.yr.1,
                        options.zr.0,
                        options.zr.1,
                    ) = match args[i + 1].parse::<f64>()
                    {
                        Ok(n) => (-n, n, -n, n, -n, n),
                        Err(_) =>
                        {
                            println!("Invalid range");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    args.remove(i);
                }
            }
            "--xr" =>
            {
                if args.len() > 2
                {
                    options.xr.0 = match args[i + 1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    options.xr.1 = match args[i + 2].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    args.remove(i);
                    args.remove(i);
                }
            }
            "--zr" =>
            {
                if args.len() > 2
                {
                    options.zr.0 = match args[i + 1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    options.zr.1 = match args[i + 2].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
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
                        Ok(x) =>
                        {
                            if !(2..=36).contains(&x)
                            {
                                println!("Invalid base");
                                err = true;
                                args.remove(i);
                                continue;
                            }
                            else
                            {
                                x
                            }
                        }
                        Err(_) =>
                        {
                            println!("Invalid base");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    args.remove(i);
                }
            }
            "--comma" => options.comma = !options.comma,
            "--sci" | "--scientific" => options.sci = !options.sci,
            "--point" =>
            {
                options.point_style = match args[i + 1].chars().next()
                {
                    Some(x) =>
                    {
                        if x == '.'
                            || x == '+'
                            || x == 'x'
                            || x == '*'
                            || x == 's'
                            || x == 'S'
                            || x == 'o'
                            || x == 'O'
                            || x == 't'
                            || x == 'T'
                            || x == 'd'
                            || x == 'D'
                            || x == 'x'
                            || x == 'R'
                        {
                            x
                        }
                        else
                        {
                            println!("Invalid point char");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    }
                    None =>
                    {
                        println!("Invalid point char");
                        err = true;
                        args.remove(i);
                        continue;
                    }
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
            "--vars" => options.allow_vars = !options.allow_vars,
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
    err
}
pub fn file_opts(options: &mut Options, file_path: &String) -> bool
{
    let mut err = false;
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
                    options.frac_iter = match split.next().unwrap().parse::<usize>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid frac iter");
                            err = true;
                            continue;
                        }
                    }
                }
                "2d" =>
                {
                    options.samples_2d = match split.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid 2d sample size");
                            err = true;
                            continue;
                        }
                    }
                }
                "3d" =>
                {
                    let mut den = split.next().unwrap().split(',');
                    if den.clone().count() != 2
                    {
                        println!("Invalid x range");
                        err = true;
                        continue;
                    }
                    options.samples_3d.0 = match den.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid 3d sample size");
                            err = true;
                            continue;
                        }
                    };
                    options.samples_3d.1 = match den.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid 3d sample size");
                            err = true;
                            continue;
                        }
                    }
                }
                "range" =>
                {
                    (
                        options.xr.0,
                        options.xr.1,
                        options.yr.0,
                        options.yr.1,
                        options.zr.0,
                        options.zr.1,
                    ) = match split.next().unwrap().parse::<f64>()
                    {
                        Ok(n) => (-n, n, -n, n, -n, n),
                        Err(_) =>
                        {
                            println!("Invalid range");
                            err = true;
                            continue;
                        }
                    };
                }
                "xr" =>
                {
                    let mut xr = split.next().unwrap().split(',');
                    if xr.clone().count() != 2
                    {
                        println!("Invalid x range");
                        err = true;
                        continue;
                    }
                    options.xr.0 = match xr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            err = true;
                            continue;
                        }
                    };
                    options.xr.1 = match xr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            err = true;
                            continue;
                        }
                    };
                }
                "yr" =>
                {
                    let mut yr = split.next().unwrap().split(',');
                    if yr.clone().count() != 2
                    {
                        println!("Invalid y range");
                        err = true;
                        continue;
                    }
                    options.yr.0 = match yr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            err = true;
                            continue;
                        }
                    };
                    options.yr.1 = match yr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            err = true;
                            continue;
                        }
                    };
                }
                "zr" =>
                {
                    let mut zr = split.next().unwrap().split(',');
                    if zr.clone().count() != 2
                    {
                        println!("Invalid z range");
                        err = true;
                        continue;
                    }
                    options.zr.0 = match zr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            err = true;
                            continue;
                        }
                    };
                    options.zr.1 = match zr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            err = true;
                            continue;
                        }
                    };
                }
                "prec" | "precision" =>
                {
                    options.prec = match split.next().unwrap().parse::<u32>()
                    {
                        Ok(x) =>
                        {
                            if x == 0
                            {
                                println!("Invalid precision");
                                err = true;
                                continue;
                            }
                            else
                            {
                                x
                            }
                        }
                        Err(_) =>
                        {
                            println!("Invalid precision");
                            err = true;
                            continue;
                        }
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
                        options.decimal_places = match r.parse::<usize>()
                        {
                            Ok(x) => x,
                            Err(_) =>
                            {
                                println!("Invalid decimal places");
                                err = true;
                                continue;
                            }
                        };
                    }
                }
                "multi" =>
                {
                    options.multi = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid multi bool");
                            err = true;
                            continue;
                        }
                    };
                }
                "depth" =>
                {
                    options.depth = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid depth bool");
                            err = true;
                            continue;
                        }
                    };
                }
                "small_e" =>
                {
                    options.small_e = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid small_e bool");
                            err = true;
                            continue;
                        }
                    };
                }
                "tabbed" =>
                {
                    options.tabbed = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid tabbed bool");
                            err = true;
                            continue;
                        }
                    };
                }
                "rt" =>
                {
                    options.real_time_output = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid real time bool");
                            err = true;
                            continue;
                        }
                    };
                }
                "line" | "lines" =>
                {
                    options.lines = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid line bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "polar" =>
                {
                    options.polar = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid polar bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "frac" =>
                {
                    options.polar = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid frac bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "prompt" =>
                {
                    options.prompt = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid prompt bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "comma" =>
                {
                    options.comma = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid comma bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "color" =>
                {
                    options.color = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid color bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "point" =>
                {
                    options.point_style = match split.next().unwrap().chars().next()
                    {
                        Some(x) =>
                        {
                            if x == '.'
                                || x == '+'
                                || x == 'x'
                                || x == '*'
                                || x == 's'
                                || x == 'S'
                                || x == 'o'
                                || x == 'O'
                                || x == 't'
                                || x == 'T'
                                || x == 'd'
                                || x == 'D'
                                || x == 'x'
                                || x == 'R'
                            {
                                x
                            }
                            else
                            {
                                println!("Invalid point char");
                                err = true;
                                continue;
                            }
                        }
                        None =>
                        {
                            println!("Invalid point char");
                            err = true;
                            continue;
                        }
                    }
                }
                "sci" | "scientific" =>
                {
                    options.sci = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid scientific bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "base" =>
                {
                    options.base = match split.next().unwrap().parse::<usize>()
                    {
                        Ok(x) =>
                        {
                            if !(2..=36).contains(&x)
                            {
                                println!("Invalid base");
                                err = true;
                                continue;
                            }
                            else
                            {
                                x
                            }
                        }
                        Err(_) =>
                        {
                            println!("Invalid base");
                            err = true;
                            continue;
                        }
                    };
                }
                "debug" =>
                {
                    options.debug = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid debug bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "deg" => options.deg = Radians,
                "rad" => options.deg = Degrees,
                "grad" => options.deg = Gradians,
                "tau" =>
                {
                    options.tau = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid tau bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "vars" =>
                {
                    options.allow_vars = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid vars bool");
                            err = true;
                            continue;
                        }
                    }
                }
                _ =>
                {}
            }
        }
    }
    err
}
pub fn equal_to(options: &mut Options, vars: &mut [[String; 2]], l: &str, last: &str)
{
    match l
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
        "xr" => println!("{},{}", options.xr.0, options.xr.1),
        "yr" => println!("{},{}", options.yr.0, options.yr.1),
        "zr" => println!("{},{}", options.zr.0, options.zr.1),
        "range" => println!(
            "x:{},{} y:{},{} z:{},{}",
            options.xr.0, options.xr.1, options.yr.0, options.yr.1, options.zr.0, options.zr.1
        ),
        "frac_iter" => println!("{}", options.frac_iter),
        "2d" => println!("{}", options.samples_2d),
        "3d" => println!("{} {}", options.samples_3d.0, options.samples_3d.1),
        _ =>
        {
            for i in match get_func(
                &input_var(
                    &l.replace('_', &format!("({})", last)),
                    vars,
                    &mut Vec::new(),
                    *options,
                ),
                *options,
            )
            {
                Ok(n) => n,
                Err(_) => return,
            }
            {
                match i
                {
                    Num(n) =>
                    {
                        let n = get_output(options, &n);
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
                            num = get_output(options, &i);
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
                                num = get_output(options, &j);
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
}
pub fn set_commands(
    options: &mut Options,
    vars: &mut [[String; 2]],
    old: &mut Vec<[String; 2]>,
    l: &str,
    r: &str,
) -> bool
{
    match l
    {
        "point" =>
        {
            if matches!(
                r,
                "." | "+" | "x" | "*" | "s" | "S" | "o" | "O" | "t" | "T" | "d" | "D" | "r" | "R"
            )
            {
                options.point_style = r.chars().next().unwrap();
            }
            else
            {
                println!("Invalid point type");
            }
            return true;
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
            return true;
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
            return true;
        }
        "prec" | "precision" =>
        {
            if r == "0"
            {
                println!("Invalid precision");
            }
            else
            {
                match r.parse::<u32>()
                {
                    Ok(n) =>
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
                    Err(_) =>
                    {
                        println!("Invalid precision");
                    }
                }
            };
            return true;
        }
        "range" =>
        {
            if r.contains(',')
            {
                (
                    options.xr.0,
                    options.xr.1,
                    options.yr.0,
                    options.yr.1,
                    options.zr.0,
                    options.zr.1,
                ) = match (
                    r.split(',').next().unwrap().parse::<f64>(),
                    r.split(',').last().unwrap().parse::<f64>(),
                )
                {
                    (Ok(min), Ok(max)) => (min, max, min, max, min, max),
                    _ =>
                    {
                        println!("Invalid range");
                        (
                            options.xr.0,
                            options.xr.1,
                            options.yr.0,
                            options.yr.1,
                            options.zr.0,
                            options.zr.1,
                        )
                    }
                }
            }
            else
            {
                (
                    options.xr.0,
                    options.xr.1,
                    options.yr.0,
                    options.yr.1,
                    options.zr.0,
                    options.zr.1,
                ) = match r.parse::<f64>()
                {
                    Ok(n) => (-n, n, -n, n, -n, n),
                    Err(_) =>
                    {
                        println!("Invalid range");
                        (
                            options.xr.0,
                            options.xr.1,
                            options.yr.0,
                            options.yr.1,
                            options.zr.0,
                            options.zr.1,
                        )
                    }
                }
            }
        }
        "xr" =>
        {
            if r.contains(',')
            {
                options.xr.0 = match r.split(',').next().unwrap().parse::<f64>()
                {
                    Ok(n) => n,
                    Err(_) =>
                    {
                        println!("Invalid x range");
                        options.xr.0
                    }
                };
                options.xr.1 = match r.split(',').last().unwrap().parse::<f64>()
                {
                    Ok(n) => n,
                    Err(_) =>
                    {
                        println!("Invalid x range");
                        options.xr.1
                    }
                };
                return true;
            }
            else
            {
                (options.xr.0, options.xr.1) = match r.parse::<f64>()
                {
                    Ok(n) => (-n, n),
                    Err(_) =>
                    {
                        println!("Invalid x range");
                        (options.xr.0, options.xr.1)
                    }
                }
            }
        }
        "yr" =>
        {
            if r.contains(',')
            {
                options.yr.0 = match r.split(',').next().unwrap().parse::<f64>()
                {
                    Ok(n) => n,
                    Err(_) =>
                    {
                        println!("Invalid y range");
                        options.yr.0
                    }
                };
                options.yr.1 = match r.split(',').last().unwrap().parse::<f64>()
                {
                    Ok(n) => n,
                    Err(_) =>
                    {
                        println!("Invalid y range");
                        options.yr.1
                    }
                };
                return true;
            }
            else
            {
                (options.yr.0, options.yr.1) = match r.parse::<f64>()
                {
                    Ok(n) => (-n, n),
                    Err(_) =>
                    {
                        println!("Invalid y range");
                        (options.yr.0, options.yr.1)
                    }
                }
            }
        }
        "zr" =>
        {
            if r.contains(',')
            {
                options.zr.0 = match r.split(',').next().unwrap().parse::<f64>()
                {
                    Ok(n) => n,
                    Err(_) =>
                    {
                        println!("Invalid z range");
                        options.zr.0
                    }
                };
                options.zr.1 = match r.split(',').last().unwrap().parse::<f64>()
                {
                    Ok(n) => n,
                    Err(_) =>
                    {
                        println!("Invalid z range");
                        options.zr.1
                    }
                };
                return true;
            }
            else
            {
                (options.zr.0, options.zr.1) = match r.parse::<f64>()
                {
                    Ok(n) => (-n, n),
                    Err(_) =>
                    {
                        println!("Invalid z range");
                        (options.zr.0, options.zr.1)
                    }
                }
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
            return true;
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
            return true;
        }
        "3d" =>
        {
            return if r.contains(',')
            {
                options.samples_3d = match (
                    r.split(',').next().unwrap().parse::<f64>(),
                    r.split(',').last().unwrap().parse::<f64>(),
                )
                {
                    (Ok(n), Ok(b)) => (n, b),
                    _ =>
                    {
                        println!("Invalid 3d sample size");
                        options.samples_3d
                    }
                };
                true
            }
            else
            {
                options.samples_3d = match r.parse::<f64>()
                {
                    Ok(n) => (n, n),
                    Err(_) =>
                    {
                        println!("Invalid 3d sample size");
                        options.samples_3d
                    }
                };
                true
            }
        }
        _ =>
        {}
    }
    false
}
pub fn commands(
    options: &mut Options,
    watch: &mut Option<Instant>,
    vars: &mut [[String; 2]],
    old: &mut Vec<[String; 2]>,
    lines: &mut Vec<String>,
    input: &mut Vec<char>,
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
            print!("\x1b[H\x1b[J");
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
            stdout.flush().unwrap();
            for l in lines
            {
                println!("{}", l);
            }
        }
        "vars" | "variables" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            for v in vars.iter()
            {
                if v[0].contains('(')
                {
                    println!("{}={}", v[0], v[1]);
                }
                else
                {
                    match &do_math(
                        get_func(&input_var(&v[1], vars, &mut Vec::new(), *options), *options)
                            .unwrap(),
                        options.deg,
                        options.prec,
                    )
                    .unwrap()
                    {
                        Num(n) =>
                        {
                            let n = get_output(options, n);
                            println!("{}={}{}", v[0], n.0, n.1)
                        }
                        Vector(m) =>
                        {
                            let mut st = String::new();
                            for i in m
                            {
                                let n = get_output(options, i);
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
                                    let n = get_output(options, g);
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
            stdout.flush().unwrap();
            for v in vars.iter()
            {
                println!("{}={}", v[0], v[1]);
            }
        }
        "version" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        }
        "exit" | "quit" | "break" =>
        {
            print!("\x1b[A\x1b[G\x1b[K");
            stdout.flush().unwrap();
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
                stdout.flush().unwrap();
                let r = split.next().unwrap();
                for i in lines
                {
                    if i.contains(r)
                    {
                        println!("{}", i);
                    }
                }
            }
            // if next == "help"
            // {
            //     print!("\x1b[A\x1b[G\x1b[K");
            //     stdout.flush().unwrap();
            //     get_help(split.next().unwrap());
            //     continue;
            // }
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