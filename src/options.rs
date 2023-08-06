use crate::{
    help::help,
    options::AngleType::{Degrees, Gradians, Radians},
    Options,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
pub fn arg_opts(options: &mut Options, args: &mut Vec<String>) -> bool {
    let mut err = false;
    args.remove(0);
    let (mut split, mut l);
    let mut i = 0;
    while i < args.len() {
        if args[i].starts_with("--") && (args[i].contains('=') || args[i].contains(',')) {
            l = args[i].clone();
            split = l.split(|c| c == '=' || c == ',');
            args[i] = split.next().unwrap().to_string();
            args.insert(i + 1, split.next().unwrap().to_string());
            if split.clone().count() > 0 {
                args.insert(i + 2, split.next().unwrap().to_string());
            }
        }
        match args[i].as_str() {
            "--debug" => options.debug = !options.debug,
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
            "--prec" | "--precision" => {
                if args.len() > 1 {
                    options.prec = match args[i + 1].parse::<u32>() {
                        Ok(x) => {
                            if x == 0 {
                                println!("Invalid precision");
                                err = true;
                                args.remove(i);
                                continue;
                            } else {
                                x
                            }
                        }
                        Err(_) => {
                            println!("Invalid precision");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    args.remove(i);
                }
            }
            "--decimal" | "--deci" | "--decimals" => {
                if args.len() > 1 {
                    if args[i + 1] == "-1" {
                        options.decimal_places = usize::MAX - 1;
                    } else if args[i + 1] == "-2" {
                        options.decimal_places = usize::MAX;
                    } else {
                        options.decimal_places = match args[i + 1].parse::<usize>() {
                            Ok(x) => x,
                            Err(_) => {
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
            "--frac_iter" => {
                if args.len() > 1 {
                    options.frac_iter = match args[i + 1].parse::<usize>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid frac iter");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    args.remove(i);
                }
            }
            "--2d" => {
                if args.len() > 1 {
                    options.samples_2d = match args[i + 1].parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid 2d sample size");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    args.remove(i);
                }
            }
            "--3d" => {
                if args.len() > 1 {
                    options.samples_3d = match args[i + 1].parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid 3d sample size");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    args.remove(i);
                }
            }
            "--yr" => {
                if args.len() > 2 {
                    options.yr[0] = match args[i + 1].parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid y range");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    options.yr[1] = match args[i + 2].parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
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
            "--xr" => {
                if args.len() > 2 {
                    options.xr[0] = match args[i + 1].parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid x range");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    options.xr[1] = match args[i + 2].parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
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
            "--zr" => {
                if args.len() > 2 {
                    options.zr[0] = match args[i + 1].parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid z range");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    };
                    options.zr[1] = match args[i + 2].parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
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
            "--base" => {
                if args.len() > 1 {
                    options.base = match args[i + 1].parse::<usize>() {
                        Ok(x) => {
                            if !(2..=36).contains(&x) {
                                println!("Invalid base");
                                err = true;
                                args.remove(i);
                                continue;
                            } else {
                                x
                            }
                        }
                        Err(_) => {
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
            "--point" => {
                options.point_style = match args[i + 1].chars().next() {
                    Some(x) => {
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
                        } else {
                            println!("Invalid point char");
                            err = true;
                            args.remove(i);
                            continue;
                        }
                    }
                    None => {
                        println!("Invalid point char");
                        err = true;
                        args.remove(i);
                        continue;
                    }
                };
                args.remove(i);
            }
            "--help" | "-h" => {
                help();
                std::process::exit(0);
            }
            "--version" | "-v" => {
                println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            "--vars" => options.allow_vars = !options.allow_vars,
            "--default" | "--def" => {
                *options = Options::default();
            }
            _ => {
                i += 1;
                continue;
            }
        }
        args.remove(i);
    }
    err
}
pub fn file_opts(options: &mut Options, file_path: &String) -> bool {
    let mut err = false;
    if File::open(file_path).is_ok() {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let mut split;
        for line in reader.lines().map(|l| l.unwrap()) {
            split = line.split('=');
            match split.next().unwrap() {
                "frac_iter" => {
                    options.frac_iter = match split.next().unwrap().parse::<usize>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid frac iter");
                            err = true;
                            continue;
                        }
                    }
                }
                "2d" => {
                    options.samples_2d = match split.next().unwrap().parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid 2d sample size");
                            err = true;
                            continue;
                        }
                    }
                }
                "3d" => {
                    options.samples_3d = match split.next().unwrap().parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid 3d sample size");
                            err = true;
                            continue;
                        }
                    }
                }
                "xr" => {
                    let mut xr = split.next().unwrap().split(',');
                    if xr.clone().count() != 2 {
                        println!("Invalid x range");
                        err = true;
                        continue;
                    }
                    options.xr[0] = match xr.next().unwrap().parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid x range");
                            err = true;
                            continue;
                        }
                    };
                    options.xr[1] = match xr.next().unwrap().parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid x range");
                            err = true;
                            continue;
                        }
                    };
                }
                "yr" => {
                    let mut yr = split.next().unwrap().split(',');
                    if yr.clone().count() != 2 {
                        println!("Invalid y range");
                        err = true;
                        continue;
                    }
                    options.yr[0] = match yr.next().unwrap().parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid y range");
                            err = true;
                            continue;
                        }
                    };
                    options.yr[1] = match yr.next().unwrap().parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid y range");
                            err = true;
                            continue;
                        }
                    };
                }
                "zr" => {
                    let mut zr = split.next().unwrap().split(',');
                    if zr.clone().count() != 2 {
                        println!("Invalid z range");
                        err = true;
                        continue;
                    }
                    options.zr[0] = match zr.next().unwrap().parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid z range");
                            err = true;
                            continue;
                        }
                    };
                    options.zr[1] = match zr.next().unwrap().parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid z range");
                            err = true;
                            continue;
                        }
                    };
                }
                "prec" | "precision" => {
                    options.prec = match split.next().unwrap().parse::<u32>() {
                        Ok(x) => {
                            if x == 0 {
                                println!("Invalid precision");
                                err = true;
                                continue;
                            } else {
                                x
                            }
                        }
                        Err(_) => {
                            println!("Invalid precision");
                            err = true;
                            continue;
                        }
                    };
                }
                "decimal" | "deci" | "decimals" => {
                    let r = split.next().unwrap();
                    if r == "-1" {
                        options.decimal_places = usize::MAX - 1;
                    } else if r == "-2" {
                        options.decimal_places = usize::MAX;
                    } else {
                        options.decimal_places = match r.parse::<usize>() {
                            Ok(x) => x,
                            Err(_) => {
                                println!("Invalid decimal places");
                                err = true;
                                continue;
                            }
                        };
                    }
                }
                "multi" => {
                    options.multi = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid multi bool");
                            err = true;
                            continue;
                        }
                    };
                }
                "small_e" => {
                    options.small_e = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid small_e bool");
                            err = true;
                            continue;
                        }
                    };
                }
                "tabbed" => {
                    options.tabbed = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid tabbed bool");
                            err = true;
                            continue;
                        }
                    };
                }
                "rt" => {
                    options.real_time_output = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid real time bool");
                            err = true;
                            continue;
                        }
                    };
                }
                "line" | "lines" => {
                    options.lines = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid line bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "polar" => {
                    options.polar = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid polar bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "frac" => {
                    options.polar = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid frac bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "prompt" => {
                    options.prompt = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid prompt bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "comma" => {
                    options.comma = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid comma bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "color" => {
                    options.color = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid color bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "point" => {
                    options.point_style = match split.next().unwrap().chars().next() {
                        Some(x) => {
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
                            } else {
                                println!("Invalid point char");
                                err = true;
                                continue;
                            }
                        }
                        None => {
                            println!("Invalid point char");
                            err = true;
                            continue;
                        }
                    }
                }
                "sci" | "scientific" => {
                    options.sci = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid scientific bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "base" => {
                    options.base = match split.next().unwrap().parse::<usize>() {
                        Ok(x) => {
                            if !(2..=36).contains(&x) {
                                println!("Invalid base");
                                err = true;
                                continue;
                            } else {
                                x
                            }
                        }
                        Err(_) => {
                            println!("Invalid base");
                            err = true;
                            continue;
                        }
                    };
                }
                "debug" => {
                    options.debug = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid debug bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "deg" => options.deg = Radians,
                "rad" => options.deg = Degrees,
                "grad" => options.deg = Gradians,
                "tau" => {
                    options.tau = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid tau bool");
                            err = true;
                            continue;
                        }
                    }
                }
                "vars" => {
                    options.allow_vars = match split.next().unwrap().parse::<bool>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Invalid vars bool");
                            err = true;
                            continue;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    err
}

#[derive(Copy, Clone, PartialEq)]
pub enum AngleType {
    Radians,
    Degrees,
    Gradians,
}
