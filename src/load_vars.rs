use crate::{
    complex::NumStr::{Num, Str},
    math::do_math,
    parse::input_var,
    Options, Variable,
};
use rug::{float::Constant::Pi, ops::CompleteRound, Complex, Float};
pub fn get_file_vars(
    options: Options,
    vars: &mut Vec<Variable>,
    lines: Vec<String>,
    r: &str,
    blacklist: &mut Vec<String>,
)
{
    if r.chars().all(|c| !c.is_alphabetic())
    {
        return;
    }
    get_preset_vars(options, r, vars, blacklist);
    'lower: for i in lines.clone()
    {
        let mut split = i.splitn(2, '=');
        if split.clone().count() == 2
        {
            let l = split.next().unwrap().to_string();
            let left = if l.contains('(')
            {
                l.split('(').next().unwrap().to_owned() + "("
            }
            else
            {
                l.clone()
            };
            if r.contains(&left) && !blacklist.contains(&left)
            {
                blacklist.push(left);
                if let Some(r) = split.next()
                {
                    let l = l.chars().collect::<Vec<char>>();
                    get_file_vars(options, vars, lines.clone(), r, blacklist);
                    for (i, j) in vars.clone().iter().enumerate()
                    {
                        if j.name.len() <= l.len()
                        {
                            add_var(l, r, i, vars, options, false, false);
                            continue 'lower;
                        }
                    }
                    add_var(l, r, 0, vars, options, false, false);
                }
            }
        }
    }
}
fn get_preset_vars(
    options: Options,
    args: &str,
    vars: &mut Vec<Variable>,
    blacklist: &mut Vec<String>,
)
{
    if args.contains("ec") && !blacklist.contains(&"ec".to_string())
    {
        blacklist.push("ec".to_string());
        vars.push(Variable {
            name: vec!['e', 'c'],
            parsed: vec![Num(Complex::parse("1.602176634e-19")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains("kB") && !blacklist.contains(&"kB".to_string())
    {
        blacklist.push("kB".to_string());
        vars.push(Variable {
            name: vec!['k', 'B'],
            parsed: vec![Num(Complex::parse("1.380649e-23")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains("me") && !blacklist.contains(&"me".to_string())
    {
        blacklist.push("me".to_string());
        vars.push(Variable {
            name: vec!['m', 'e'],
            parsed: vec![Num(Complex::parse("9.1093837015e-31")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains("mn") && !blacklist.contains(&"mn".to_string())
    {
        blacklist.push("mn".to_string());
        vars.push(Variable {
            name: vec!['m', 'n'],
            parsed: vec![Num(Complex::parse("1.67492749804e-27")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains("mp") && !blacklist.contains(&"mp".to_string())
    {
        blacklist.push("mp".to_string());
        vars.push(Variable {
            name: vec!['m', 'p'],
            parsed: vec![Num(Complex::parse("1.67262192369e-27")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains("Na") && !blacklist.contains(&"Na".to_string())
    {
        blacklist.push("Na".to_string());
        vars.push(Variable {
            name: vec!['N', 'a'],
            parsed: vec![Num(Complex::parse("6.02214076e23")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains('c') && !blacklist.contains(&"c".to_string())
    {
        blacklist.push("c".to_string());
        vars.push(Variable {
            name: vec!['c'],
            parsed: vec![Num(Complex::parse("299792458")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains('G') && !blacklist.contains(&"G".to_string())
    {
        blacklist.push("G".to_string());
        vars.push(Variable {
            name: vec!['G'],
            parsed: vec![Num(Complex::parse("6.67430e-11")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains('g') && !blacklist.contains(&"g".to_string())
    {
        blacklist.push("g".to_string());
        vars.push(Variable {
            name: vec!['g'],
            parsed: vec![Num(Complex::parse("9.80665")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains('h') && !blacklist.contains(&"h".to_string())
    {
        blacklist.push("h".to_string());
        vars.push(Variable {
            name: vec!['h'],
            parsed: vec![Num(Complex::parse("6.62607015e-34")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains('k') && !blacklist.contains(&"k".to_string())
    {
        blacklist.push("k".to_string());
        vars.push(Variable {
            name: vec!['k'],
            parsed: vec![Num(Complex::parse("8.9875517923e9")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains('R') && !blacklist.contains(&"R".to_string())
    {
        blacklist.push("R".to_string());
        vars.push(Variable {
            name: vec!['R'],
            parsed: vec![Num(Complex::parse("8.31446261815324")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    {
        let phi1 = args.contains("phi") && !blacklist.contains(&"phi".to_string());
        let phi2 = args.contains('φ') && !blacklist.contains(&"φ".to_string());
        if phi1 || phi2
        {
            let phi: Float = (1 + Float::with_val(options.prec.0, 5).sqrt()) / 2;
            if phi1
            {
                blacklist.push("phi".to_string());
                vars.insert(
                    0,
                    Variable {
                        name: vec!['p', 'h', 'i'],
                        parsed: vec![Num(phi.clone().into())],
                        unparsed: String::new(),
                    },
                );
            }
            if phi2
            {
                blacklist.push("φ".to_string());
                vars.push(Variable {
                    name: vec!['φ'],
                    parsed: vec![Num(phi.into())],
                    unparsed: String::new(),
                });
            }
        }
    }
    {
        let pi1 = args.contains("pi") && !blacklist.contains(&"pi".to_string());
        let pi2 = args.contains('π') && !blacklist.contains(&"π".to_string());
        let tau1 = args.contains("tau") && !blacklist.contains(&"tau".to_string());
        let tau2 = args.contains('τ') && !blacklist.contains(&"τ".to_string());
        if pi1 || pi2 || tau1 || tau2
        {
            let pi = Float::with_val(options.prec.0, Pi);
            if pi1
            {
                blacklist.push("pi".to_string());
                vars.insert(
                    vars.iter().position(|c| c.name.len() != 3).unwrap_or(0),
                    Variable {
                        name: vec!['p', 'i'],
                        parsed: vec![Num(pi.clone().into())],
                        unparsed: String::new(),
                    },
                );
            }
            if pi2
            {
                blacklist.push("π".to_string());
                vars.push(Variable {
                    name: vec!['π'],
                    parsed: vec![Num(pi.clone().into())],
                    unparsed: String::new(),
                });
            }
            if tau1 || tau2
            {
                let tau: Float = pi.clone() * 2;
                if tau1
                {
                    blacklist.push("tau".to_string());
                    vars.insert(
                        0,
                        Variable {
                            name: vec!['t', 'a', 'u'],
                            parsed: vec![Num(tau.clone().into())],
                            unparsed: String::new(),
                        },
                    );
                }
                if tau2
                {
                    blacklist.push("τ".to_string());
                    vars.push(Variable {
                        name: vec!['τ'],
                        parsed: vec![Num(tau.into())],
                        unparsed: String::new(),
                    });
                }
            }
        }
    }
    if args.contains('e') && !blacklist.contains(&"e".to_string())
    {
        blacklist.push("e".to_string());
        let e = Float::with_val(options.prec.0, 1).exp();
        vars.push(Variable {
            name: vec!['e'],
            parsed: vec![Num(e.into())],
            unparsed: String::new(),
        });
    }
}
pub fn get_cli_vars(options: Options, args: String, vars: &mut Vec<Variable>)
{
    if args.chars().all(|c| !c.is_alphabetic())
    {
        return;
    }
    if args.contains("ec")
    {
        vars.push(Variable {
            name: vec!['e', 'c'],
            parsed: vec![Num(Complex::parse("1.602176634e-19")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains("kB")
    {
        vars.push(Variable {
            name: vec!['k', 'B'],
            parsed: vec![Num(Complex::parse("1.380649e-23")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains("me")
    {
        vars.push(Variable {
            name: vec!['m', 'e'],
            parsed: vec![Num(Complex::parse("9.1093837015e-31")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains("mn")
    {
        vars.push(Variable {
            name: vec!['m', 'n'],
            parsed: vec![Num(Complex::parse("1.67492749804e-27")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains("mp")
    {
        vars.push(Variable {
            name: vec!['m', 'p'],
            parsed: vec![Num(Complex::parse("1.67262192369e-27")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains("Na")
    {
        vars.push(Variable {
            name: vec!['N', 'a'],
            parsed: vec![Num(Complex::parse("6.02214076e23")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains('c')
    {
        vars.push(Variable {
            name: vec!['c'],
            parsed: vec![Num(Complex::parse("299792458")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains('G')
    {
        vars.push(Variable {
            name: vec!['G'],
            parsed: vec![Num(Complex::parse("6.67430e-11")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains('g')
    {
        vars.push(Variable {
            name: vec!['g'],
            parsed: vec![Num(Complex::parse("9.80665")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains('h')
    {
        vars.push(Variable {
            name: vec!['h'],
            parsed: vec![Num(Complex::parse("6.62607015e-34")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains('k')
    {
        vars.push(Variable {
            name: vec!['k'],
            parsed: vec![Num(Complex::parse("8.9875517923e9")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    if args.contains('R')
    {
        vars.push(Variable {
            name: vec!['R'],
            parsed: vec![Num(Complex::parse("8.31446261815324")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        });
    }
    {
        let phi1 = args.contains("phi");
        let phi2 = args.contains('φ');
        if phi1 || phi2
        {
            let phi: Float = (1 + Float::with_val(options.prec.0, 5).sqrt()) / 2;
            if phi1
            {
                vars.insert(
                    0,
                    Variable {
                        name: vec!['p', 'h', 'i'],
                        parsed: vec![Num(phi.clone().into())],
                        unparsed: String::new(),
                    },
                );
            }
            if phi2
            {
                vars.push(Variable {
                    name: vec!['φ'],
                    parsed: vec![Num(phi.into())],
                    unparsed: String::new(),
                });
            }
        }
    }
    {
        let pi1 = args.contains("pi");
        let pi2 = args.contains('π');
        let tau1 = args.contains("tau");
        let tau2 = args.contains('τ');
        if pi1 || pi2 || tau1 || tau2
        {
            let pi = Float::with_val(options.prec.0, Pi);
            if pi1
            {
                vars.insert(
                    vars.iter().position(|c| c.name.len() != 3).unwrap_or(0),
                    Variable {
                        name: vec!['p', 'i'],
                        parsed: vec![Num(pi.clone().into())],
                        unparsed: String::new(),
                    },
                );
            }
            if pi2
            {
                vars.push(Variable {
                    name: vec!['π'],
                    parsed: vec![Num(pi.clone().into())],
                    unparsed: String::new(),
                });
            }
            if tau1 || tau2
            {
                let tau: Float = pi.clone() * 2;
                if tau1
                {
                    vars.insert(
                        0,
                        Variable {
                            name: vec!['t', 'a', 'u'],
                            parsed: vec![Num(tau.clone().into())],
                            unparsed: String::new(),
                        },
                    );
                }
                if tau2
                {
                    vars.push(Variable {
                        name: vec!['τ'],
                        parsed: vec![Num(tau.into())],
                        unparsed: String::new(),
                    });
                }
            }
        }
    }
    if args.contains('e')
    {
        let e = Float::with_val(options.prec.0, 1).exp();
        vars.push(Variable {
            name: vec!['e'],
            parsed: vec![Num(e.into())],
            unparsed: String::new(),
        });
    }
}
pub fn get_vars(options: Options) -> Vec<Variable>
{
    let pi = Float::with_val(options.prec.0, Pi);
    let tau: Float = pi.clone() * 2;
    let phi: Float = (1 + Float::with_val(options.prec.0, 5).sqrt()) / 2;
    let e = Float::with_val(options.prec.0, 1).exp();
    vec![
        Variable {
            name: vec!['p', 'h', 'i'],
            parsed: vec![Num(phi.clone().into())],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['t', 'a', 'u'],
            parsed: vec![Num(tau.clone().into())],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['e', 'c'],
            parsed: vec![Num(Complex::parse("1.602176634e-19")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['k', 'B'],
            parsed: vec![Num(Complex::parse("1.380649e-23")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['m', 'e'],
            parsed: vec![Num(Complex::parse("9.1093837015e-31")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['m', 'n'],
            parsed: vec![Num(Complex::parse("1.67492749804e-27")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['m', 'p'],
            parsed: vec![Num(Complex::parse("1.67262192369e-27")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['N', 'a'],
            parsed: vec![Num(Complex::parse("6.02214076e23")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['p', 'i'],
            parsed: vec![Num(pi.clone().into())],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['c'],
            parsed: vec![Num(Complex::parse("299792458")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['e'],
            parsed: vec![Num(e.into())],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['G'],
            parsed: vec![Num(Complex::parse("6.67430e-11")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['g'],
            parsed: vec![Num(Complex::parse("9.80665")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['h'],
            parsed: vec![Num(Complex::parse("6.62607015e-34")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['k'],
            parsed: vec![Num(Complex::parse("8.9875517923e9")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['R'],
            parsed: vec![Num(Complex::parse("8.31446261815324")
                .unwrap()
                .complete(options.prec))],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['φ'],
            parsed: vec![Num(phi.into())],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['π'],
            parsed: vec![Num(pi.into())],
            unparsed: String::new(),
        },
        Variable {
            name: vec!['τ'],
            parsed: vec![Num(tau.into())],
            unparsed: String::new(),
        },
    ]
}
pub fn add_var(
    l: Vec<char>,
    r: &str,
    i: usize,
    vars: &mut Vec<Variable>,
    options: Options,
    redef: bool,
    replace: bool,
)
{
    let mut func_vars: Vec<(isize, String)> = Vec::new();
    if l.contains(&'(')
    {
        let mut l = l.clone();
        l.drain(0..=l.iter().position(|c| c == &'(').unwrap());
        l.pop();
        for i in l.split(|c| c == &',')
        {
            func_vars.push((-1, i.iter().collect()));
        }
    }
    vars.push(Variable {
        name: l.clone(),
        parsed: vec![Str(String::new())],
        unparsed: String::new(),
    });
    let parsed = match input_var(
        r,
        vars.clone(),
        &mut func_vars,
        &mut 0,
        options,
        false,
        &mut (false, 0, 0),
        false,
        0,
        l.clone(),
    )
    {
        Ok(n) => n.0,
        _ =>
        {
            println!("failed at: {}\x1b[G", l.iter().collect::<String>());
            return;
        }
    };
    vars.pop();
    if replace
    {
        vars[i] = Variable {
            name: l.clone(),
            parsed: if l.contains(&'(')
            {
                parsed
            }
            else
            {
                vec![do_math(parsed, options, Vec::new()).unwrap_or(Num(Complex::new(options.prec)))]
            },
            unparsed: r.to_string(),
        };
    }
    else
    {
        vars.insert(
            i,
            Variable {
                name: l.clone(),
                parsed: if l.contains(&'(')
                {
                    parsed
                }
                else
                {
                    vec![do_math(parsed, options, Vec::new())
                        .unwrap_or(Num(Complex::new(options.prec)))]
                },
                unparsed: r.to_string(),
            },
        )
    }
    if redef
    {
        let mut redef = vec![l.clone()];
        let mut k = 0;
        while k < redef.len()
        {
            for (j, v) in vars.clone().iter().enumerate()
            {
                if redef[k] != v.name
                    && v.unparsed.contains(
                        &redef[k][0..=redef[k]
                            .iter()
                            .position(|a| a == &'(')
                            .unwrap_or(redef[k].len() - 1)]
                            .iter()
                            .collect::<String>(),
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
                            func_vars.push((-1, i.iter().collect::<String>()));
                        }
                    }
                    let parsed = match input_var(
                        &v.unparsed.clone(),
                        vars.clone(),
                        &mut func_vars,
                        &mut 0,
                        options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                        v.name.clone(),
                    )
                    {
                        Ok(n) => n.0,
                        _ =>
                        {
                            println!("failed at: {}\x1b[G", v.name.iter().collect::<String>());
                            return;
                        }
                    };
                    let check = vars[j].parsed.clone();
                    if v.name.contains(&'(')
                    {
                        vars[j].parsed = parsed.clone();
                    }
                    else
                    {
                        vars[j].parsed = vec![do_math(parsed.clone(), options, Vec::new())
                            .unwrap_or(Num(Complex::new(options.prec)))];
                    }
                    if check != parsed
                    {
                        redef.push(v.name.clone());
                    }
                }
            }
            k += 1;
        }
    }
}