use crate::{
    complex::NumStr::Num, math::do_math, options::set_commands, parse::input_var, Colors, Options,
    Variable,
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
                l.split('(').next().unwrap().to_owned()
            }
            else
            {
                l.clone()
            };
            if if l.contains('(')
            {
                r.contains(&(left.clone() + "("))
                    || r.contains(&(left.clone() + "{"))
                    || r.contains(&(left.clone() + "["))
            }
            else
            {
                r.contains(&left)
            } && !blacklist.contains(&l)
            {
                if let Some(r) = split.next()
                {
                    blacklist.push(l.clone());
                    let l = l.chars().collect::<Vec<char>>();
                    get_file_vars(options, vars, lines.clone(), r, blacklist);
                    for (i, j) in vars.clone().iter().enumerate()
                    {
                        if j.name.len() <= l.len()
                        {
                            if let Err(s) = add_var(l, r, i, vars, options, false, false, false)
                            {
                                println!("{}", s)
                            }
                            continue 'lower;
                        }
                    }
                    if let Err(s) = add_var(l, r, 0, vars, options, false, false, false)
                    {
                        println!("{}", s)
                    }
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
    let prec = (options.prec, options.prec);
    if args.contains("ec") && !blacklist.contains(&"ec".to_string())
    {
        blacklist.push("ec".to_string());
        vars.push(Variable {
            name: vec!['e', 'c'],
            parsed: vec![Num(Complex::parse("1.602176634e-19")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains("kB") && !blacklist.contains(&"kB".to_string())
    {
        blacklist.push("kB".to_string());
        vars.push(Variable {
            name: vec!['k', 'B'],
            parsed: vec![Num(Complex::parse("1.380649e-23").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains("me") && !blacklist.contains(&"me".to_string())
    {
        blacklist.push("me".to_string());
        vars.push(Variable {
            name: vec!['m', 'e'],
            parsed: vec![Num(Complex::parse("9.1093837015e-31")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains("mn") && !blacklist.contains(&"mn".to_string())
    {
        blacklist.push("mn".to_string());
        vars.push(Variable {
            name: vec!['m', 'n'],
            parsed: vec![Num(Complex::parse("1.67492749804e-27")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains("mp") && !blacklist.contains(&"mp".to_string())
    {
        blacklist.push("mp".to_string());
        vars.push(Variable {
            name: vec!['m', 'p'],
            parsed: vec![Num(Complex::parse("1.67262192369e-27")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains("Na") && !blacklist.contains(&"Na".to_string())
    {
        blacklist.push("Na".to_string());
        vars.push(Variable {
            name: vec!['N', 'a'],
            parsed: vec![Num(Complex::parse("6.02214076e23").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains('c') && !blacklist.contains(&"c".to_string())
    {
        blacklist.push("c".to_string());
        vars.push(Variable {
            name: vec!['c'],
            parsed: vec![Num(Complex::parse("299792458").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains('G') && !blacklist.contains(&"G".to_string())
    {
        blacklist.push("G".to_string());
        vars.push(Variable {
            name: vec!['G'],
            parsed: vec![Num(Complex::parse("6.67430e-11").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains('g') && !blacklist.contains(&"g".to_string())
    {
        blacklist.push("g".to_string());
        vars.push(Variable {
            name: vec!['g'],
            parsed: vec![Num(Complex::parse("9.80665").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains('h') && !blacklist.contains(&"h".to_string())
    {
        blacklist.push("h".to_string());
        vars.push(Variable {
            name: vec!['h'],
            parsed: vec![Num(Complex::parse("6.62607015e-34")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains('k') && !blacklist.contains(&"k".to_string())
    {
        blacklist.push("k".to_string());
        vars.push(Variable {
            name: vec!['k'],
            parsed: vec![Num(Complex::parse("8.9875517923e9")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains('R') && !blacklist.contains(&"R".to_string())
    {
        blacklist.push("R".to_string());
        vars.push(Variable {
            name: vec!['R'],
            parsed: vec![Num(Complex::parse("8.31446261815324")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    {
        let phi1 = args.contains("phi") && !blacklist.contains(&"phi".to_string());
        let phi2 = args.contains('φ') && !blacklist.contains(&"φ".to_string());
        if phi1 || phi2
        {
            let phi: Float = (1 + Float::with_val(options.prec, 5).sqrt()) / 2;
            if phi1
            {
                blacklist.push("phi".to_string());
                vars.insert(
                    0,
                    Variable {
                        name: vec!['p', 'h', 'i'],
                        parsed: vec![Num(phi.clone().into())],
                        unparsed: String::new(),
                        funcvars: Vec::new(),
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
                    funcvars: Vec::new(),
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
            let pi = Float::with_val(options.prec, Pi);
            if pi1
            {
                blacklist.push("pi".to_string());
                vars.insert(
                    vars.iter().position(|c| c.name.len() != 3).unwrap_or(0),
                    Variable {
                        name: vec!['p', 'i'],
                        parsed: vec![Num(pi.clone().into())],
                        unparsed: String::new(),
                        funcvars: Vec::new(),
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
                    funcvars: Vec::new(),
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
                            funcvars: Vec::new(),
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
                        funcvars: Vec::new(),
                    });
                }
            }
        }
    }
    if args.contains('e') && !blacklist.contains(&"e".to_string())
    {
        blacklist.push("e".to_string());
        let e = Float::with_val(options.prec, 1).exp();
        vars.push(Variable {
            name: vec!['e'],
            parsed: vec![Num(e.into())],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
}
pub fn get_cli_vars(options: Options, args: String, vars: &mut Vec<Variable>)
{
    if args.chars().all(|c| !c.is_alphabetic())
    {
        return;
    }
    let prec = (options.prec, options.prec);
    if args.contains("ec")
    {
        vars.push(Variable {
            name: vec!['e', 'c'],
            parsed: vec![Num(Complex::parse("1.602176634e-19")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains("kB")
    {
        vars.push(Variable {
            name: vec!['k', 'B'],
            parsed: vec![Num(Complex::parse("1.380649e-23").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains("me")
    {
        vars.push(Variable {
            name: vec!['m', 'e'],
            parsed: vec![Num(Complex::parse("9.1093837015e-31")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains("mn")
    {
        vars.push(Variable {
            name: vec!['m', 'n'],
            parsed: vec![Num(Complex::parse("1.67492749804e-27")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains("mp")
    {
        vars.push(Variable {
            name: vec!['m', 'p'],
            parsed: vec![Num(Complex::parse("1.67262192369e-27")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains("Na")
    {
        vars.push(Variable {
            name: vec!['N', 'a'],
            parsed: vec![Num(Complex::parse("6.02214076e23").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains('c')
    {
        vars.push(Variable {
            name: vec!['c'],
            parsed: vec![Num(Complex::parse("299792458").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains('G')
    {
        vars.push(Variable {
            name: vec!['G'],
            parsed: vec![Num(Complex::parse("6.67430e-11").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains('g')
    {
        vars.push(Variable {
            name: vec!['g'],
            parsed: vec![Num(Complex::parse("9.80665").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains('h')
    {
        vars.push(Variable {
            name: vec!['h'],
            parsed: vec![Num(Complex::parse("6.62607015e-34")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains('k')
    {
        vars.push(Variable {
            name: vec!['k'],
            parsed: vec![Num(Complex::parse("8.9875517923e9")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    if args.contains('R')
    {
        vars.push(Variable {
            name: vec!['R'],
            parsed: vec![Num(Complex::parse("8.31446261815324")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
    {
        let phi1 = args.contains("phi");
        let phi2 = args.contains('φ');
        if phi1 || phi2
        {
            let phi: Float = (1 + Float::with_val(options.prec, 5).sqrt()) / 2;
            if phi1
            {
                vars.insert(
                    0,
                    Variable {
                        name: vec!['p', 'h', 'i'],
                        parsed: vec![Num(phi.clone().into())],
                        unparsed: String::new(),
                        funcvars: Vec::new(),
                    },
                );
            }
            if phi2
            {
                vars.push(Variable {
                    name: vec!['φ'],
                    parsed: vec![Num(phi.into())],
                    unparsed: String::new(),
                    funcvars: Vec::new(),
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
            let pi = Float::with_val(options.prec, Pi);
            if pi1
            {
                vars.insert(
                    vars.iter().position(|c| c.name.len() != 3).unwrap_or(0),
                    Variable {
                        name: vec!['p', 'i'],
                        parsed: vec![Num(pi.clone().into())],
                        unparsed: String::new(),
                        funcvars: Vec::new(),
                    },
                );
            }
            if pi2
            {
                vars.push(Variable {
                    name: vec!['π'],
                    parsed: vec![Num(pi.clone().into())],
                    unparsed: String::new(),
                    funcvars: Vec::new(),
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
                            funcvars: Vec::new(),
                        },
                    );
                }
                if tau2
                {
                    vars.push(Variable {
                        name: vec!['τ'],
                        parsed: vec![Num(tau.into())],
                        unparsed: String::new(),
                        funcvars: Vec::new(),
                    });
                }
            }
        }
    }
    if args.contains('e')
    {
        let e = Float::with_val(options.prec, 1).exp();
        vars.push(Variable {
            name: vec!['e'],
            parsed: vec![Num(e.into())],
            unparsed: String::new(),
            funcvars: Vec::new(),
        });
    }
}
pub fn get_vars(options: Options) -> Vec<Variable>
{
    let prec = (options.prec, options.prec);
    let pi = Float::with_val(options.prec, Pi);
    let tau: Float = pi.clone() * 2;
    let phi: Float = (1 + Float::with_val(options.prec, 5).sqrt()) / 2;
    let e = Float::with_val(options.prec, 1).exp();
    vec![
        Variable {
            name: vec!['p', 'h', 'i'],
            parsed: vec![Num(phi.clone().into())],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['t', 'a', 'u'],
            parsed: vec![Num(tau.clone().into())],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['e', 'c'],
            parsed: vec![Num(Complex::parse("1.602176634e-19")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['k', 'B'],
            parsed: vec![Num(Complex::parse("1.380649e-23").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['m', 'e'],
            parsed: vec![Num(Complex::parse("9.1093837015e-31")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['m', 'n'],
            parsed: vec![Num(Complex::parse("1.67492749804e-27")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['m', 'p'],
            parsed: vec![Num(Complex::parse("1.67262192369e-27")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['N', 'a'],
            parsed: vec![Num(Complex::parse("6.02214076e23").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['p', 'i'],
            parsed: vec![Num(pi.clone().into())],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['c'],
            parsed: vec![Num(Complex::parse("299792458").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['e'],
            parsed: vec![Num(e.into())],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['G'],
            parsed: vec![Num(Complex::parse("6.67430e-11").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['g'],
            parsed: vec![Num(Complex::parse("9.80665").unwrap().complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['h'],
            parsed: vec![Num(Complex::parse("6.62607015e-34")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['k'],
            parsed: vec![Num(Complex::parse("8.9875517923e9")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['R'],
            parsed: vec![Num(Complex::parse("8.31446261815324")
                .unwrap()
                .complete(prec))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['φ'],
            parsed: vec![Num(phi.into())],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['π'],
            parsed: vec![Num(pi.into())],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['τ'],
            parsed: vec![Num(tau.into())],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
    ]
}
#[allow(clippy::too_many_arguments)]
pub fn add_var(
    l: Vec<char>,
    mut r: &str,
    i: usize,
    vars: &mut Vec<Variable>,
    options: Options,
    redef: bool,
    replace: bool,
    null: bool,
) -> Result<(), &'static str>
{
    if null
    {
        vars.remove(i);
    }
    else
    {
        let orig = r;
        let mut func_vars: Vec<(isize, String)> = Vec::new();
        if l.contains(&'(')
        {
            let mut l = l.clone();
            if l.drain(0..=l.iter().position(|c| c == &'(').unwrap())
                .collect::<String>()
                .contains(',')
            {
                return Err("bad var name");
            }
            l.pop();
            for i in l.split(|c| c == &',')
            {
                func_vars.push((-1, i.iter().collect()));
            }
        }
        else if l.contains(&',')
        {
            return Err("bad var name");
        }
        let mut k = 0;
        for (j, v) in vars.iter().enumerate()
        {
            if v.name.len() <= l.len()
            {
                k = j;
                break;
            }
        }
        let mut tempvars = vars.clone();
        tempvars.insert(
            k,
            Variable {
                name: l.clone(),
                parsed: Vec::new(),
                unparsed: String::new(),
                funcvars: Vec::new(),
            },
        );
        let mut fvs = Vec::new();
        if r.contains(':')
        {
            let mut split = r.split(':').collect::<Vec<&str>>();
            r = split.pop().unwrap();
            for i in split
            {
                if i.contains('=')
                {
                    let mut split = i.splitn(2, '=');
                    let s = split.next().unwrap().to_string();
                    let parsed = input_var(
                        split.next().unwrap(),
                        tempvars.clone(),
                        &mut func_vars,
                        &mut 0,
                        options,
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
        let mut parsed = input_var(
            r,
            tempvars.clone(),
            &mut func_vars,
            &mut 0,
            options,
            false,
            false,
            0,
            l.clone(),
        )?;
        parsed.1.extend(fvs);
        if parsed.0.is_empty()
        {
            return Err("bad input");
        }
        else if replace
        {
            vars[i] = Variable {
                name: l.clone(),
                parsed: if l.contains(&'(')
                {
                    parsed.0
                }
                else
                {
                    vec![do_math(parsed.0, options, parsed.1.clone())?]
                },
                unparsed: orig.to_string(),
                funcvars: parsed.1,
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
                        parsed.0
                    }
                    else
                    {
                        vec![do_math(parsed.0, options, parsed.1.clone())?]
                    },
                    unparsed: orig.to_string(),
                    funcvars: parsed.1,
                },
            )
        }
        if l.contains(&'(')
            && r.contains(l.split(|c| c == &'(').next().unwrap())
            && (r.contains("piecewise") || r.contains("pw"))
        {
            let parsed = vars[i].parsed.clone();
            vars[i]
                .funcvars
                .push((l.iter().collect::<String>(), parsed))
        }
    }
    if redef
    {
        let mut redef = vec![l.clone()];
        let mut k = 0;
        while k < redef.len()
        {
            for (j, v) in vars.clone().iter().enumerate()
            {
                let check = &redef[k][0..=redef[k]
                    .iter()
                    .position(|a| a == &'(')
                    .unwrap_or(redef[k].len() - 1)]
                    .iter()
                    .collect::<String>();
                if redef[k] != v.name && v.unparsed.contains(check)
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
                    let mut fvs = Vec::new();
                    let mut unparsed = v.unparsed.clone();
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
                                    vars.clone(),
                                    &mut func_vars,
                                    &mut 0,
                                    options,
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
                    let mut parsed = input_var(
                        &unparsed,
                        vars.clone(),
                        &mut func_vars,
                        &mut 0,
                        options,
                        false,
                        false,
                        0,
                        v.name.clone(),
                    )?;
                    parsed.1.extend(fvs);
                    if v.name.contains(&'(')
                    {
                        if vars[j].parsed.clone() != parsed.0
                            || vars[j].funcvars.clone() != parsed.1
                        {
                            redef.push(v.name.clone());
                            vars[j].parsed.clone_from(&parsed.0);
                            vars[j].funcvars.clone_from(&parsed.1);
                        }
                    }
                    else if let Ok(n) = do_math(parsed.0.clone(), options, parsed.1.clone())
                    {
                        if n != vars[j].parsed[0]
                        {
                            redef.push(v.name.clone());
                            vars[j].parsed = vec![n];
                        }
                    }
                    if v.name.contains(&'(')
                        && v.unparsed
                            .contains(v.name.split(|c| c == &'(').next().unwrap())
                        && (v.unparsed.contains("piecewise") || v.unparsed.contains("pw"))
                    {
                        let parsed = vars[j].parsed.clone();
                        vars[j]
                            .funcvars
                            .push((v.name.iter().collect::<String>(), parsed))
                    }
                }
            }
            k += 1;
        }
    }
    Ok(())
}
pub fn set_commands_or_vars(
    colors: &mut Colors,
    options: &mut Options,
    vars: &mut Vec<Variable>,
    input: &[char],
) -> Result<(), &'static str>
{
    let n = input.iter().collect::<String>();
    let mut split = n.splitn(2, '=');
    let s = split.next().unwrap().replace(' ', "");
    let l = s;
    let r = split.next().unwrap();
    if l.is_empty()
        || l.chars()
            .any(|c| !c.is_alphanumeric() && !matches!(c, '(' | ')' | ',' | '\'' | '`'))
    {
        return Ok(());
    }
    else if let Err(s) = set_commands(options, colors, vars, &l, r)
    {
        if s.is_empty()
        {
            return Ok(());
        }
        return Err(s);
    }
    let l = l.chars().collect::<Vec<char>>();
    for (i, v) in vars.iter().enumerate()
    {
        if v.name.split(|c| c == &'(').next() == l.split(|c| c == &'(').next()
            && v.name.contains(&'(') == l.contains(&'(')
            && v.name.iter().filter(|c| c == &&',').count()
                == l.iter().filter(|c| c == &&',').count()
        {
            if r == "null"
            {
                add_var(l, r, i, vars, *options, true, true, true)?
            }
            else
            {
                add_var(l, r, i, vars, *options, true, true, false)?
            }
            return Ok(());
        }
    }
    for (i, j) in vars.iter().enumerate()
    {
        if j.name.len() <= l.len()
        {
            add_var(l, r, i, vars, *options, true, false, false)?;
            return Ok(());
        }
    }
    add_var(l, r, 0, vars, *options, true, false, false)?;
    Ok(())
}
