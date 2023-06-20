use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::{GraphOptions, help, PrintOptions};
pub fn arg_opts(graph_options:&mut GraphOptions, print_options:&mut PrintOptions, allow_vars:&mut bool, debug:&mut bool, prec:&mut u32, args:&mut Vec<String>) -> bool
{
    let mut err = false;
    args.remove(0);
    let (mut split, mut l);
    while !args.is_empty()
    {
        if args[0].contains('-') && (args[0].contains('=') || args[0].contains(','))
        {
            l = args[0].clone();
            split = l.split(|c| c == '=' || c == ',');
            args[0] = split.next().unwrap().to_string();
            args.insert(1, split.next().unwrap().to_string());
            if split.clone().count() > 0
            {
                args.insert(2, split.next().unwrap().to_string());
            }
        }
        match args[0].as_str()
        {
            "--debug" => *debug = !*debug,
            "--tau" => print_options.tau = !print_options.tau,
            "--deg" => print_options.deg = !print_options.deg,
            "--prompt" => print_options.prompt = !print_options.prompt,
            "--color" => print_options.color = !print_options.color,
            "--line" => graph_options.lines = !graph_options.lines,
            "--rt" => print_options.real_time_output = !print_options.real_time_output,
            "--polar" => print_options.polar = !print_options.polar,
            "--prec" | "--precision" =>
            {
                if args.len() > 1
                {
                    *prec = match args[1].parse::<u32>()
                    {
                        Ok(x) =>
                        {
                            if x == 0
                            {
                                println!("Invalid precision");
                                err = true;
                                args.remove(0);
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
                            args.remove(0);
                            continue;
                        }
                    };
                    args.remove(0);
                }
            }
            "--decimal" | "--deci" | "--decimals" =>
            {
                if args.len() > 1
                {
                    print_options.decimal_places = match args[1].parse::<usize>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid decimal");
                            err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    args.remove(0);
                }
            }
            "--2d" =>
            {
                if args.len() > 1
                {
                    graph_options.samples_2d = match args[1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid 2d sample size");
                            err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    args.remove(0);
                }
            }
            "--3d" =>
            {
                if args.len() > 1
                {
                    graph_options.samples_3d = match args[1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid 3d sample size");
                            err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    args.remove(0);
                }
            }
            "--yr" =>
            {
                if args.len() > 2
                {
                    graph_options.yr[0] = match args[1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    graph_options.yr[1] = match args[2].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    args.remove(0);
                    args.remove(0);
                }
            }
            "--xr" =>
            {
                if args.len() > 2
                {
                    graph_options.xr[0] = match args[1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    graph_options.xr[1] = match args[2].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    args.remove(0);
                    args.remove(0);
                }
            }
            "--zr" =>
            {
                if args.len() > 2
                {
                    graph_options.zr[0] = match args[1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    graph_options.zr[1] = match args[2].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    args.remove(0);
                    args.remove(0);
                }
            }
            "--base" =>
            {
                if args.len() > 1
                {
                    print_options.base = match args[1].parse::<usize>()
                    {
                        Ok(x) =>
                        {
                            if !(2..=36).contains(&x)
                            {
                                println!("Invalid base");
                                err = true;
                                args.remove(0);
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
                            args.remove(0);
                            continue;
                        }
                    };
                    args.remove(0);
                }
            }
            "--comma" => print_options.comma = !print_options.comma,
            "--sci" | "--scientific" => print_options.sci = !print_options.sci,
            "--point" =>
            {
                graph_options.point_style = match args[1].chars().next()
                {
                    Some(x) =>
                    {
                        if x == '.' || x == '+' || x == 'x' || x == '*' || x == 's' || x == 'S' || x == 'o' || x == 'O' || x == 't' || x == 'T' || x == 'd' || x == 'D' || x == 'x' || x == 'R'
                        {
                            x
                        }
                        else
                        {
                            println!("Invalid point char");
                            err = true;
                            args.remove(0);
                            continue;
                        }
                    }
                    None =>
                    {
                        println!("Invalid point char");
                        err = true;
                        args.remove(0);
                        continue;
                    }
                };
                args.remove(0);
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
            "--vars" => *allow_vars = !*allow_vars,
            "--default" | "--def" =>
            {
                *print_options = PrintOptions::default();
                *graph_options = GraphOptions::default();
                *prec = 256;
                *allow_vars = true;
                *debug = false;
            }
            _ => break,
        }
        args.remove(0);
    }
    err
}
pub fn file_opts(graph_options:&mut GraphOptions, print_options:&mut PrintOptions, allow_vars:&mut bool, debug:&mut bool, prec:&mut u32, file_path:&String) -> bool
{
    let mut err = false;
    if File::open(file_path).is_ok()
    {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let mut split;
        for line in reader.lines().map(|l| l.unwrap())
        {
            split = line.split('=');
            match split.next().unwrap()
            {
                "2d" =>
                {
                    graph_options.samples_2d = match split.next().unwrap().parse::<f64>()
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
                    graph_options.samples_3d = match split.next().unwrap().parse::<f64>()
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
                "xr" =>
                {
                    let mut xr = split.next().unwrap().split(',');
                    if xr.clone().count() != 2
                    {
                        println!("Invalid x range");
                        err = true;
                        continue;
                    }
                    graph_options.xr[0] = match xr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            err = true;
                            continue;
                        }
                    };
                    graph_options.xr[1] = match xr.next().unwrap().parse::<f64>()
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
                    graph_options.yr[0] = match yr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            err = true;
                            continue;
                        }
                    };
                    graph_options.yr[1] = match yr.next().unwrap().parse::<f64>()
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
                    graph_options.zr[0] = match zr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            err = true;
                            continue;
                        }
                    };
                    graph_options.zr[1] = match zr.next().unwrap().parse::<f64>()
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
                    *prec = match split.next().unwrap().parse::<u32>()
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
                    print_options.decimal_places = match split.next().unwrap().parse::<usize>()
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
                "rt" =>
                {
                    print_options.real_time_output = match split.next().unwrap().parse::<bool>()
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
                "line" =>
                {
                    graph_options.lines = match split.next().unwrap().parse::<bool>()
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
                    print_options.polar = match split.next().unwrap().parse::<bool>()
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
                "prompt" =>
                {
                    print_options.prompt = match split.next().unwrap().parse::<bool>()
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
                    print_options.comma = match split.next().unwrap().parse::<bool>()
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
                    print_options.color = match split.next().unwrap().parse::<bool>()
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
                    graph_options.point_style = match split.next().unwrap().chars().next()
                    {
                        Some(x) =>
                        {
                            if x == '.' || x == '+' || x == 'x' || x == '*' || x == 's' || x == 'S' || x == 'o' || x == 'O' || x == 't' || x == 'T' || x == 'd' || x == 'D' || x == 'x' || x == 'R'
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
                    print_options.sci = match split.next().unwrap().parse::<bool>()
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
                    print_options.base = match split.next().unwrap().parse::<usize>()
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
                    *debug = match split.next().unwrap().parse::<bool>()
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
                "deg" =>
                {
                    print_options.deg = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid degree bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "tau" =>
                {
                    print_options.tau = match split.next().unwrap().parse::<bool>()
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
                    *allow_vars = match split.next().unwrap().parse::<bool>()
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