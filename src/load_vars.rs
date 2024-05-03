use crate::{
    complex::NumStr::Num, math::do_math, options::set_commands, parse::input_var, Colors, Number,
    Options, Units, Variable,
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
fn ev(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['e', 'V'],
        parsed: vec![Num(Number::from(
            Complex::parse("1.602176634e-19").unwrap().complete(prec),
            Some(Units {
                meter: 2.0,
                second: -2.0,
                kilogram: 1.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
    }
}
fn ec(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['e', 'C'],
        parsed: vec![Num(Number::from(
            Complex::parse("1.602176634e-19").unwrap().complete(prec),
            Some(Units {
                ampere: 1.0,
                second: 1.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
    }
}
fn kb(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['b', 'o', 'l', 't', 'z', 'm', 'a', 'n', 'n'],
        parsed: vec![Num(Number::from(
            Complex::parse("1.380649e-23").unwrap().complete(prec),
            Some(Units {
                second: -2.0,
                meter: 2.0,
                kilogram: 1.0,
                kelvin: -1.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
    }
}
fn me(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['e', 'M'],
        parsed: vec![Num(Number::from(
            Complex::parse("9.1093837015e-31").unwrap().complete(prec),
            Some(Units {
                kilogram: 1.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
    }
}
fn mn(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['n', 'M'],
        parsed: vec![Num(Number::from(
            Complex::parse("1.67492749804e-27").unwrap().complete(prec),
            Some(Units {
                kilogram: 1.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
    }
}
fn mp(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['p', 'M'],
        parsed: vec![Num(Number::from(
            Complex::parse("1.67262192369e-27").unwrap().complete(prec),
            Some(Units {
                kilogram: 1.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
    }
}
fn c(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['c'],
        parsed: vec![Num(Number::from(
            Complex::parse("299792458").unwrap().complete(prec),
            Some(Units {
                second: -1.0,
                meter: 1.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
    }
}
fn g(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['g', 'r', 'a', 'v', 'i', 't', 'y'],
        parsed: vec![Num(Number::from(
            Complex::parse("9.80665").unwrap().complete(prec),
            Some(Units {
                second: -2.0,
                meter: 1.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
    }
}
fn gc(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['G'],
        parsed: vec![Num(Number::from(
            Complex::parse("6.67430e-11").unwrap().complete(prec),
            Some(Units {
                second: -2.0,
                meter: 3.0,
                kilogram: -1.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
    }
}
fn h(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['p', 'l', 'a', 'n', 'c', 'k'],
        parsed: vec![Num(Number::from(
            Complex::parse("6.62607015e-34").unwrap().complete(prec),
            Some(Units {
                second: -1.0,
                meter: 2.0,
                kilogram: 1.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
    }
}
fn na(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['N', 'a'],
        parsed: vec![Num(Number::from(
            Complex::parse("6.02214076e23").unwrap().complete(prec),
            Some(Units {
                mole: -1.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
    }
}
fn k(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['k', 'e'],
        parsed: vec![Num(Number::from(
            Complex::parse("8987551792.3").unwrap().complete(prec),
            Some(Units {
                second: -4.0,
                meter: 3.0,
                kilogram: 1.0,
                ampere: -2.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
    }
}
fn r(prec: (u32, u32)) -> Variable
{
    Variable {
        name: vec!['R'],
        parsed: vec![Num(Number::from(
            Complex::parse("8.31446261815324").unwrap().complete(prec),
            Some(Units {
                second: -2.0,
                meter: 2.0,
                kilogram: 1.0,
                kelvin: 1.0,
                mole: -1.0,
                ..Units::default()
            }),
        ))],
        unparsed: String::new(),
        funcvars: Vec::new(),
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
    if args.contains("boltzmann") && !blacklist.contains(&"boltzmann".to_string())
    {
        blacklist.push("boltzmann".to_string());
        vars.push(kb(prec));
    }
    if args.contains("gravity") && !blacklist.contains(&"gravity".to_string())
    {
        blacklist.push("gravity".to_string());
        vars.push(g(prec));
    }
    if args.contains("planck") && !blacklist.contains(&"planck".to_string())
    {
        blacklist.push("planck".to_string());
        vars.push(h(prec));
    }
    if args.contains("ke") && !blacklist.contains(&"ke".to_string())
    {
        blacklist.push("ke".to_string());
        vars.push(k(prec));
    }
    if args.contains("eV") && !blacklist.contains(&"eV".to_string())
    {
        blacklist.push("eV".to_string());
        vars.push(ev(prec));
    }
    if args.contains("eC") && !blacklist.contains(&"eC".to_string())
    {
        blacklist.push("eC".to_string());
        vars.push(ec(prec));
    }
    if args.contains("eM") && !blacklist.contains(&"eM".to_string())
    {
        blacklist.push("eM".to_string());
        vars.push(me(prec));
    }
    if args.contains("nM") && !blacklist.contains(&"nM".to_string())
    {
        blacklist.push("nM".to_string());
        vars.push(mn(prec));
    }
    if args.contains("pM") && !blacklist.contains(&"pM".to_string())
    {
        blacklist.push("pM".to_string());
        vars.push(mp(prec));
    }
    if args.contains("Na") && !blacklist.contains(&"Na".to_string())
    {
        blacklist.push("Na".to_string());
        vars.push(na(prec));
    }
    if args.contains('c') && !blacklist.contains(&"c".to_string())
    {
        blacklist.push("c".to_string());
        vars.push(c(prec));
    }
    if args.contains('G') && !blacklist.contains(&"G".to_string())
    {
        blacklist.push("G".to_string());
        vars.push(gc(prec));
    }
    if args.contains('R') && !blacklist.contains(&"R".to_string())
    {
        blacklist.push("R".to_string());
        vars.push(r(prec));
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
                        parsed: vec![Num(Number::from(phi.clone().into(), None))],
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
                    parsed: vec![Num(Number::from(phi.into(), None))],
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
                        parsed: vec![Num(Number::from(pi.clone().into(), None))],
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
                    parsed: vec![Num(Number::from(pi.clone().into(), None))],
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
                            parsed: vec![Num(Number::from(tau.clone().into(), None))],
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
                        parsed: vec![Num(Number::from(tau.into(), None))],
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
            parsed: vec![Num(Number::from(e.into(), None))],
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
    if args.contains("boltzmann")
    {
        vars.push(kb(prec));
    }
    if args.contains("gravity")
    {
        vars.push(g(prec));
    }
    if args.contains("planck")
    {
        vars.push(h(prec));
    }
    if args.contains("ke")
    {
        vars.push(k(prec));
    }
    if args.contains("eV")
    {
        vars.push(ev(prec));
    }
    if args.contains("eC")
    {
        vars.push(ec(prec));
    }
    if args.contains("eM")
    {
        vars.push(me(prec));
    }
    if args.contains("nM")
    {
        vars.push(mn(prec));
    }
    if args.contains("pM")
    {
        vars.push(mp(prec));
    }
    if args.contains("Na")
    {
        vars.push(na(prec));
    }
    if args.contains('c')
    {
        vars.push(c(prec));
    }
    if args.contains('G')
    {
        vars.push(gc(prec));
    }
    if args.contains('R')
    {
        vars.push(r(prec));
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
                        parsed: vec![Num(Number::from(phi.clone().into(), None))],
                        unparsed: String::new(),
                        funcvars: Vec::new(),
                    },
                );
            }
            if phi2
            {
                vars.push(Variable {
                    name: vec!['φ'],
                    parsed: vec![Num(Number::from(phi.into(), None))],
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
                        parsed: vec![Num(Number::from(pi.clone().into(), None))],
                        unparsed: String::new(),
                        funcvars: Vec::new(),
                    },
                );
            }
            if pi2
            {
                vars.push(Variable {
                    name: vec!['π'],
                    parsed: vec![Num(Number::from(pi.clone().into(), None))],
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
                            parsed: vec![Num(Number::from(tau.clone().into(), None))],
                            unparsed: String::new(),
                            funcvars: Vec::new(),
                        },
                    );
                }
                if tau2
                {
                    vars.push(Variable {
                        name: vec!['τ'],
                        parsed: vec![Num(Number::from(tau.into(), None))],
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
            parsed: vec![Num(Number::from(e.into(), None))],
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
        kb(prec),
        g(prec),
        h(prec),
        Variable {
            name: vec!['p', 'h', 'i'],
            parsed: vec![Num(Number::from(phi.clone().into(), None))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['t', 'a', 'u'],
            parsed: vec![Num(Number::from(tau.clone().into(), None))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        ec(prec),
        ev(prec),
        me(prec),
        mn(prec),
        mp(prec),
        k(prec),
        na(prec),
        Variable {
            name: vec!['p', 'i'],
            parsed: vec![Num(Number::from(pi.clone().into(), None))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        c(prec),
        Variable {
            name: vec!['e'],
            parsed: vec![Num(Number::from(e.into(), None))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        gc(prec),
        r(prec),
        Variable {
            name: vec!['φ'],
            parsed: vec![Num(Number::from(phi.into(), None))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['π'],
            parsed: vec![Num(Number::from(pi.into(), None))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['τ'],
            parsed: vec![Num(Number::from(tau.into(), None))],
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
            l.pop();
            let st = l
                .drain(0..=l.iter().position(|c| c == &'(').unwrap())
                .collect::<String>();
            if st.contains(',') || st.len() == 1
            {
                return Err("bad var name");
            }
            for i in l.split(|c| c == &',')
            {
                func_vars.push((-1, i.iter().collect()));
            }
        }
        else if l.contains(&',') || l.is_empty()
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
                        &tempvars,
                        &mut func_vars,
                        &mut 0,
                        options,
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
            &tempvars,
            &mut func_vars,
            &mut 0,
            options,
            false,
            0,
            l.clone(),
        )?;
        parsed.1.extend(fvs);
        if l.contains(&'(')
            && r.contains(l.split(|c| c == &'(').next().unwrap())
            && (r.contains("piecewise") || r.contains("pw"))
        {
            parsed
                .1
                .push((l.iter().collect::<String>(), parsed.0.clone()))
        }
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
                    .unwrap_or(redef[k].len().saturating_sub(1))]
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
                                    vars,
                                    &mut func_vars,
                                    &mut 0,
                                    options,
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
                        vars,
                        &mut func_vars,
                        &mut 0,
                        options,
                        false,
                        0,
                        v.name.clone(),
                    )?;
                    parsed.1.extend(fvs);
                    if v.name.contains(&'(')
                        && v.unparsed
                            .contains(v.name.split(|c| c == &'(').next().unwrap())
                        && (v.unparsed.contains("piecewise") || v.unparsed.contains("pw"))
                    {
                        parsed
                            .1
                            .push((v.name.iter().collect::<String>(), parsed.0.clone()));
                    }
                    if v.name.contains(&'(')
                    {
                        if vars[j].parsed != parsed.0 || vars[j].funcvars != parsed.1
                        {
                            redef.push(v.name.clone());
                            vars[j].parsed = parsed.0;
                            vars[j].funcvars = parsed.1;
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
    add_var(l, r, vars.len(), vars, *options, true, false, false)?;
    Ok(())
}
