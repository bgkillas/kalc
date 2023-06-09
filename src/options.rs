use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::help;
#[allow(clippy::too_many_arguments)]
pub fn arg_opts(graph_options:&mut ([[f64; 2]; 3], f64, f64, char, bool),
                print_options:&mut (bool, bool, usize, bool, bool, usize),
                allow_vars:&mut bool,
                debug:&mut bool,
                prompt:&mut bool,
                color:&mut bool,
                prec:&mut u32,
                err:&mut bool,
                args:&mut Vec<String>)
{
    args.remove(0);
    let (mut split, mut l);
    while !args.is_empty()
    {
        if args[0].contains('=') || args[0].contains(',')
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
            "--tau" => print_options.3 = !print_options.3,
            "--deg" => print_options.1 = !print_options.1,
            "--prompt" => *prompt = !*prompt,
            "--color" => *color = !*color,
            "--line" => graph_options.4 = !graph_options.4,
            "--rt" => print_options.4 = !print_options.4,
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
                                *err = true;
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
                            *err = true;
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
                    print_options.5 = match args[1].parse::<usize>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid decimal");
                            *err = true;
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
                    graph_options.1 = match args[1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid 2d sample size");
                            *err = true;
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
                    graph_options.2 = match args[1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid 3d sample size");
                            *err = true;
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
                    graph_options.0[1][0] = match args[1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            *err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    graph_options.0[1][1] = match args[2].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            *err = true;
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
                    graph_options.0[0][0] = match args[1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            *err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    graph_options.0[0][1] = match args[2].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            *err = true;
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
                    graph_options.0[2][0] = match args[1].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            *err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    graph_options.0[2][1] = match args[2].parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            *err = true;
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
                    print_options.2 = match args[1].parse::<usize>()
                    {
                        Ok(x) =>
                        {
                            if !(2..=36).contains(&x)
                            {
                                println!("Invalid base");
                                *err = true;
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
                            *err = true;
                            args.remove(0);
                            continue;
                        }
                    };
                    args.remove(0);
                }
            }
            "--sci" | "--scientific" => print_options.0 = !print_options.0,
            "--point" =>
            {
                graph_options.3 = match args[1].chars().next()
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
                            *err = true;
                            args.remove(0);
                            continue;
                        }
                    }
                    None =>
                    {
                        println!("Invalid point char");
                        *err = true;
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
                *print_options = (false, false, 10, false, true, 12);
                *graph_options = ([[-10.0, 10.0]; 3], 40000.0, 400.0, '.', false);
                *prec = 256;
                *allow_vars = true;
                *debug = false;
                *prompt = true;
                *color = true;
            }
            _ => break,
        }
        args.remove(0);
    }
}
#[allow(clippy::too_many_arguments)]
pub fn file_opts(graph_options:&mut ([[f64; 2]; 3], f64, f64, char, bool),
                 print_options:&mut (bool, bool, usize, bool, bool, usize),
                 allow_vars:&mut bool,
                 debug:&mut bool,
                 prompt:&mut bool,
                 color:&mut bool,
                 prec:&mut u32,
                 err:&mut bool,
                 file_path:&String)
{
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
                    graph_options.1 = match split.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid 2d sample size");
                            *err = true;
                            continue;
                        }
                    }
                }
                "3d" =>
                {
                    graph_options.2 = match split.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid 3d sample size");
                            *err = true;
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
                        *err = true;
                        continue;
                    }
                    graph_options.0[0][0] = match xr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            *err = true;
                            continue;
                        }
                    };
                    graph_options.0[0][1] = match xr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid x range");
                            *err = true;
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
                        *err = true;
                        continue;
                    }
                    graph_options.0[1][0] = match yr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            *err = true;
                            continue;
                        }
                    };
                    graph_options.0[1][1] = match yr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid y range");
                            *err = true;
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
                        *err = true;
                        continue;
                    }
                    graph_options.0[2][0] = match zr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            *err = true;
                            continue;
                        }
                    };
                    graph_options.0[2][1] = match zr.next().unwrap().parse::<f64>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid z range");
                            *err = true;
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
                                *err = true;
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
                            *err = true;
                            continue;
                        }
                    };
                }
                "decimal" | "deci" | "decimals" =>
                {
                    print_options.5 = match split.next().unwrap().parse::<usize>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid decimal places");
                            *err = true;
                            continue;
                        }
                    };
                }
                "rt" =>
                {
                    print_options.4 = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid real time bool");
                            *err = true;
                            continue;
                        }
                    };
                }
                "line" =>
                {
                    graph_options.4 = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid line bool");
                            *err = true;
                            continue;
                        }
                    }
                }
                "prompt" =>
                {
                    *prompt = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid prompt bool");
                            *err = true;
                            continue;
                        }
                    }
                }
                "color" =>
                {
                    *color = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid color bool");
                            *err = true;
                            continue;
                        }
                    }
                }
                "point" =>
                {
                    graph_options.3 = match split.next().unwrap().chars().next()
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
                                *err = true;
                                continue;
                            }
                        }
                        None =>
                        {
                            println!("Invalid point char");
                            *err = true;
                            continue;
                        }
                    }
                }
                "sci" | "scientific" =>
                {
                    print_options.0 = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid scientific bool");
                            *err = true;
                            continue;
                        }
                    }
                }
                "base" =>
                {
                    print_options.2 = match split.next().unwrap().parse::<usize>()
                    {
                        Ok(x) =>
                        {
                            if !(2..=36).contains(&x)
                            {
                                println!("Invalid base");
                                *err = true;
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
                            *err = true;
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
                            *err = true;
                            continue;
                        }
                    }
                }
                "deg" =>
                {
                    print_options.1 = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid degree bool");
                            *err = true;
                            continue;
                        }
                    }
                }
                "tau" =>
                {
                    print_options.3 = match split.next().unwrap().parse::<bool>()
                    {
                        Ok(x) => x,
                        Err(_) =>
                        {
                            println!("Invalid tau bool");
                            *err = true;
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
                            *err = true;
                            continue;
                        }
                    }
                }
                _ =>
                {}
            }
        }
    }
}