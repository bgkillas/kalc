use crate::{
    complex::{
        NumStr,
        NumStr::{Num, Str},
    },
    math::do_math,
    parse::input_var,
    Options,
};
use rug::{float::Constant::Pi, ops::CompleteRound, Complex, Float};
pub fn get_file_vars(
    options: Options,
    vars: &mut Vec<(Vec<char>, Vec<NumStr>, NumStr, String)>,
    lines: Vec<String>,
    r: &str,
    blacklist: &mut Vec<String>,
)
{
    'lower: for i in lines.clone()
    {
        //TODO fix n(x) with 'e',make another function that makes a vec of new, needed vars
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
                        if j.0.len() <= l.len()
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
pub fn get_cli_vars(
    options: Options,
    args: String,
    vars: &mut Vec<(Vec<char>, Vec<NumStr>, NumStr, String)>,
)
{
    //TODO maybe scrap and have it load it in load_vars
    if args.chars().all(|c| !c.is_alphabetic())
    {
        return;
    }
    if args.contains("ec")
    {
        vars.push((
            vec!['e', 'c'],
            Vec::new(),
            Num(Complex::parse("1.602176634e-19")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains("kB")
    {
        vars.push((
            vec!['k', 'B'],
            Vec::new(),
            Num(Complex::parse("1.380649e-23")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains("me")
    {
        vars.push((
            vec!['m', 'e'],
            Vec::new(),
            Num(Complex::parse("9.1093837015e-31")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains("mn")
    {
        vars.push((
            vec!['m', 'n'],
            Vec::new(),
            Num(Complex::parse("1.67492749804e-27")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains("mp")
    {
        vars.push((
            vec!['m', 'p'],
            Vec::new(),
            Num(Complex::parse("1.67262192369e-27")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains("Na")
    {
        vars.push((
            vec!['N', 'a'],
            Vec::new(),
            Num(Complex::parse("6.02214076e23")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains('c')
    {
        vars.push((
            vec!['c'],
            Vec::new(),
            Num(Complex::parse("299792458").unwrap().complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains('G')
    {
        vars.push((
            vec!['G'],
            Vec::new(),
            Num(Complex::parse("6.67430e-11")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains('g')
    {
        vars.push((
            vec!['g'],
            Vec::new(),
            Num(Complex::parse("9.80665").unwrap().complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains('h')
    {
        vars.push((
            vec!['h'],
            Vec::new(),
            Num(Complex::parse("6.62607015e-34")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains('k')
    {
        vars.push((
            vec!['k'],
            Vec::new(),
            Num(Complex::parse("8.9875517923e9")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains('R')
    {
        vars.push((
            vec!['R'],
            Vec::new(),
            Num(Complex::parse("8.31446261815324")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ));
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
                    (
                        vec!['p', 'h', 'i'],
                        Vec::new(),
                        Num(phi.clone().into()),
                        "".to_string(),
                    ),
                )
            }
            if phi2
            {
                vars.push((vec!['φ'], Vec::new(), Num(phi.into()), "".to_string()))
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
                    0,
                    (
                        vec!['p', 'i'],
                        Vec::new(),
                        Num(pi.clone().into()),
                        "".to_string(),
                    ),
                );
            }
            if pi2
            {
                vars.push((
                    vec!['π'],
                    Vec::new(),
                    Num(pi.clone().into()),
                    "".to_string(),
                ))
            }
            if tau1 || tau2
            {
                let tau: Float = pi.clone() * 2;
                if tau1
                {
                    vars.insert(
                        0,
                        (
                            vec!['t', 'a', 'u'],
                            Vec::new(),
                            Num(tau.clone().into()),
                            "".to_string(),
                        ),
                    );
                }
                if tau2
                {
                    vars.push((vec!['τ'], Vec::new(), Num(tau.into()), "".to_string()))
                }
            }
        }
    }
    if args.contains('e')
    {
        let e = Float::with_val(options.prec.0, 1).exp();
        vars.push((vec!['e'], Vec::new(), Num(e.into()), "".to_string()))
    }
}
pub fn get_vars(options: Options) -> Vec<(Vec<char>, Vec<NumStr>, NumStr, String)>
{
    let pi = Float::with_val(options.prec.0, Pi);
    let tau: Float = pi.clone() * 2;
    let phi: Float = (1 + Float::with_val(options.prec.0, 5).sqrt()) / 2;
    let e = Float::with_val(options.prec.0, 1).exp();
    vec![
        (
            vec!['p', 'h', 'i'],
            Vec::new(),
            Num(phi.clone().into()),
            "".to_string(),
        ),
        (
            vec!['t', 'a', 'u'],
            Vec::new(),
            Num(tau.clone().into()),
            "".to_string(),
        ),
        (
            vec!['e', 'c'],
            Vec::new(),
            Num(Complex::parse("1.602176634e-19")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            vec!['k', 'B'],
            Vec::new(),
            Num(Complex::parse("1.380649e-23")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            vec!['m', 'e'],
            Vec::new(),
            Num(Complex::parse("9.1093837015e-31")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            vec!['m', 'n'],
            Vec::new(),
            Num(Complex::parse("1.67492749804e-27")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            vec!['m', 'p'],
            Vec::new(),
            Num(Complex::parse("1.67262192369e-27")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            vec!['N', 'a'],
            Vec::new(),
            Num(Complex::parse("6.02214076e23")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            vec!['p', 'i'],
            Vec::new(),
            Num(pi.clone().into()),
            "".to_string(),
        ),
        (
            vec!['c'],
            Vec::new(),
            Num(Complex::parse("299792458").unwrap().complete(options.prec)),
            "".to_string(),
        ),
        (vec!['e'], Vec::new(), Num(e.into()), "".to_string()),
        (
            vec!['G'],
            Vec::new(),
            Num(Complex::parse("6.67430e-11")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            vec!['g'],
            Vec::new(),
            Num(Complex::parse("9.80665").unwrap().complete(options.prec)),
            "".to_string(),
        ),
        (
            vec!['h'],
            Vec::new(),
            Num(Complex::parse("6.62607015e-34")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            vec!['k'],
            Vec::new(),
            Num(Complex::parse("8.9875517923e9")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            vec!['R'],
            Vec::new(),
            Num(Complex::parse("8.31446261815324")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (vec!['φ'], Vec::new(), Num(phi.into()), "".to_string()),
        (vec!['π'], Vec::new(), Num(pi.into()), "".to_string()),
        (vec!['τ'], Vec::new(), Num(tau.into()), "".to_string()),
    ]
}
pub fn add_var(
    l: Vec<char>,
    r: &str,
    i: usize,
    vars: &mut Vec<(Vec<char>, Vec<NumStr>, NumStr, String)>,
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
    if r.contains("piecewise")
    {
        vars.push((l.clone(), Vec::new(), Str(String::new()), String::new()));
    }
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
    )
    {
        Ok(n) => n.0,
        _ =>
        {
            println!("failed at: {}", l.iter().collect::<String>());
            return;
        }
    };
    if l.contains(&'(')
    {
        vars.insert(i, (l.clone(), parsed, Str(String::new()), r.to_string()));
    }
    else
    {
        vars.insert(
            i,
            (
                l.clone(),
                Vec::new(),
                do_math(parsed, options, Vec::new()).unwrap_or(Num(Complex::new(options.prec))),
                r.to_string(),
            ),
        );
    }
    if replace
    {
        vars.remove(i + 1);
    }
    if redef
    {
        let mut redef = vec![l.clone()];
        let mut k = 0;
        while k < redef.len()
        {
            for (j, v) in vars.clone().iter().enumerate()
            {
                if redef[k] != v.0
                    && v.3.contains(
                        &redef[k][0..=redef[k]
                            .iter()
                            .position(|a| a == &'(')
                            .unwrap_or(redef[k].len() - 1)]
                            .iter()
                            .collect::<String>(),
                    )
                {
                    let mut func_vars: Vec<(isize, String)> = Vec::new();
                    if v.0.contains(&'(')
                    {
                        let mut l = v.0.clone();
                        l.drain(0..=l.iter().position(|c| c == &'(').unwrap());
                        l.pop();
                        for i in l.split(|c| c == &',')
                        {
                            func_vars.push((-1, i.iter().collect::<String>()));
                        }
                    }
                    let parsed = match input_var(
                        &v.3.clone(),
                        vars.clone(),
                        &mut func_vars,
                        &mut 0,
                        options,
                        false,
                        &mut (false, 0, 0),
                        false,
                        0,
                    )
                    {
                        Ok(n) => n.0,
                        _ =>
                        {
                            println!("failed at: {}", v.0.iter().collect::<String>());
                            return;
                        }
                    };
                    let check = vars[j].1.clone();
                    if v.0.contains(&'(')
                    {
                        vars[j] = (v.0.clone(), parsed.clone(), Str(String::new()), v.3.clone());
                    }
                    else
                    {
                        vars[j] = (
                            v.0.clone(),
                            Vec::new(),
                            do_math(parsed.clone(), options, Vec::new())
                                .unwrap_or(Num(Complex::new(options.prec))),
                            v.3.clone(),
                        );
                    }
                    if check != parsed
                    {
                        redef.push(v.0.clone());
                    }
                }
            }
            k += 1;
        }
    }
}