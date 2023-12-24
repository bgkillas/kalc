use crate::{
    complex::{NumStr, NumStr::Num},
    Options,
};
use rug::{float::Constant::Pi, ops::CompleteRound, Complex, Float};
pub fn get_cli_vars(options: Options, args: String)
    -> Vec<(Vec<char>, Vec<NumStr>, NumStr, String)>
{
    let mut vars = Vec::new();
    if args.chars().all(|c| !c.is_alphabetic())
    {
        return vars;
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
    vars
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