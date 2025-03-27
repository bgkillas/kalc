use crate::{
    complex::NumStr::Num,
    math::do_math,
    options::set_commands,
    parse::input_var,
    units::{Colors, Number, Options, Variable},
};
use rug::{Float, float::Constant::Pi};
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
            if !blacklist.contains(&l) && {
                let mut word = String::new();
                let mut b = false;
                for c in r.chars()
                {
                    if c.is_alphanumeric() || matches!(c, '\'' | '`' | '_')
                    {
                        word.push(c)
                    }
                    else
                    {
                        if l.contains('(')
                        {
                            b = word.trim_end_matches('\'').trim_end_matches('`') == left
                                && matches!(c, '(' | '{' | '[' | '|');
                        }
                        else
                        {
                            b = word == left;
                        }
                        if b
                        {
                            break;
                        }
                        word.clear()
                    }
                }
                b
            }
            {
                if let Some(r) = split.next()
                {
                    let le = l.chars().collect::<Vec<char>>();
                    blacklist.push(l.clone());
                    get_file_vars(options, vars, lines.clone(), r, blacklist);
                    for (i, j) in vars.iter().enumerate()
                    {
                        if j.name.len() <= le.len()
                        {
                            if let Err(s) = add_var(le, r, i, vars, options, false, false, false)
                            {
                                println!("{}", s)
                            }
                            continue 'lower;
                        }
                    }
                    if let Err(s) = add_var(le, r, 0, vars, options, false, false, false)
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
    let pi = Float::with_val(options.prec, Pi);
    let tau: Float = pi.clone() * 2;
    let phi: Float = (1 + Float::with_val(options.prec, 5).sqrt()) / 2;
    let e = Float::with_val(options.prec, 1).exp();
    vec![
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
        Variable {
            name: vec!['p', 'i'],
            parsed: vec![Num(Number::from(pi.clone().into(), None))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
        Variable {
            name: vec!['e'],
            parsed: vec![Num(Number::from(e.into(), None))],
            unparsed: String::new(),
            funcvars: Vec::new(),
        },
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
                .drain(0..=l.iter().position(|c| c == &'(').unwrap_or(0))
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
        let mut fvs = Vec::new();
        let mut parsed = if r.contains("pw") || r.contains("piecewise")
        {
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
            input_var(
                r,
                &tempvars,
                &mut func_vars,
                &mut 0,
                options,
                false,
                0,
                l.clone(),
                false,
                &mut Vec::new(),
                None,
            )?
        }
        else
        {
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
                            vars,
                            &mut func_vars,
                            &mut 0,
                            options,
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
            input_var(
                r,
                vars,
                &mut func_vars,
                &mut 0,
                options,
                false,
                0,
                l.clone(),
                false,
                &mut Vec::new(),
                None,
            )?
        };
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
        let mut redef = vec![(l.clone(), Vec::new())];
        let mut k = 0;
        while k < redef.len()
        {
            let mut j = 0;
            while j < vars.len()
            {
                let v = &vars[j];
                let check = &redef[k].0[0..=redef[k]
                    .0
                    .iter()
                    .position(|a| a == &'(')
                    .unwrap_or(redef[k].0.len().saturating_sub(1))]
                    .iter()
                    .collect::<String>();
                if !redef.iter().any(|a| a.0 == v.name && a.1 == redef[k].0)
                    && v.unparsed.contains(check)
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
                    let mut parsed = input_var(
                        &unparsed,
                        vars,
                        &mut func_vars,
                        &mut 0,
                        options,
                        false,
                        0,
                        v.name.clone(),
                        false,
                        &mut Vec::new(),
                        None,
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
                        if v.parsed != parsed.0 || v.funcvars != parsed.1
                        {
                            redef.push((v.name.clone(), redef[k].0.clone()));
                            vars[j].parsed = parsed.0;
                            vars[j].funcvars = parsed.1;
                        }
                    }
                    else if let Ok(n) = do_math(parsed.0.clone(), options, parsed.1.clone())
                    {
                        if n != v.parsed[0]
                        {
                            redef.push((v.name.clone(), redef[k].0.clone()));
                            vars[j].parsed = vec![n];
                        }
                    }
                }
                j += 1;
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
            .any(|c| !c.is_alphanumeric() && !matches!(c, '(' | ')' | ',' | '\'' | '`' | '_'))
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
