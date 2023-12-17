use crate::{
    complex::{NumStr, NumStr::Num},
    Options,
};
use rug::{float::Constant::Pi, ops::CompleteRound, Complex, Float};
pub fn get_cli_vars(options: Options, args: &[String]) -> Vec<(String, String, NumStr)>
{
    let mut vars = Vec::new();
    let args = args.concat();
    if args.chars().all(|c| !c.is_alphabetic())
    {
        return vars;
    }
    if args.contains("ec")
    {
        vars.push((
            "ec".to_string(),
            "1.602176634e-19".to_string(),
            Num(Complex::parse("1.602176634e-19")
                .unwrap()
                .complete((options.prec, options.prec))),
        ));
    }
    if args.contains("kB")
    {
        vars.push((
            "kB".to_string(),
            "1.380649e-23".to_string(),
            Num(Complex::parse("1.380649e-23")
                .unwrap()
                .complete((options.prec, options.prec))),
        ));
    }
    if args.contains("me")
    {
        vars.push((
            "me".to_string(),
            "9.1093837015e-31".to_string(),
            Num(Complex::parse("9.1093837015e-31")
                .unwrap()
                .complete((options.prec, options.prec))),
        ));
    }
    if args.contains("mn")
    {
        vars.push((
            "mn".to_string(),
            "1.67492749804e-27".to_string(),
            Num(Complex::parse("1.67492749804e-27")
                .unwrap()
                .complete((options.prec, options.prec))),
        ));
    }
    if args.contains("mp")
    {
        vars.push((
            "mp".to_string(),
            "1.67262192369e-27".to_string(),
            Num(Complex::parse("1.67262192369e-27")
                .unwrap()
                .complete((options.prec, options.prec))),
        ));
    }
    if args.contains("Na")
    {
        vars.push((
            "Na".to_string(),
            "6.02214076e23".to_string(),
            Num(Complex::parse("6.02214076e23")
                .unwrap()
                .complete((options.prec, options.prec))),
        ));
    }
    if args.contains('c')
    {
        vars.push((
            "c".to_string(),
            "299792458".to_string(),
            Num(Complex::parse("299792458")
                .unwrap()
                .complete((options.prec, options.prec))),
        ));
    }
    if args.contains('G')
    {
        vars.push((
            "G".to_string(),
            "6.67430e-11".to_string(),
            Num(Complex::parse("6.67430e-11")
                .unwrap()
                .complete((options.prec, options.prec))),
        ));
    }
    if args.contains('g')
    {
        vars.push((
            "g".to_string(),
            "9.80665".to_string(),
            Num(Complex::parse("9.80665")
                .unwrap()
                .complete((options.prec, options.prec))),
        ));
    }
    if args.contains('h')
    {
        vars.push((
            "h".to_string(),
            "6.62607015e-34".to_string(),
            Num(Complex::parse("6.62607015e-34")
                .unwrap()
                .complete((options.prec, options.prec))),
        ));
    }
    if args.contains('k')
    {
        vars.push((
            "k".to_string(),
            "8.9875517923e9".to_string(),
            Num(Complex::parse("8.9875517923e9")
                .unwrap()
                .complete((options.prec, options.prec))),
        ));
    }
    if args.contains('R')
    {
        vars.push((
            "R".to_string(),
            "8.31446261815324".to_string(),
            Num(Complex::parse("8.31446261815324")
                .unwrap()
                .complete((options.prec, options.prec))),
        ));
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
                    (
                        "phi".to_string(),
                        "phi".to_string(),
                        Num(phi.clone().into()),
                    ),
                )
            }
            if phi2
            {
                vars.push(("φ".to_string(), "phi".to_string(), Num(phi.into())))
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
                    0,
                    ("pi".to_string(), "pi".to_string(), Num(pi.clone().into())),
                );
            }
            if pi2
            {
                vars.push(('π'.to_string(), "pi".to_string(), Num(pi.clone().into())))
            }
            if tau1 || tau2
            {
                let tau: Float = pi.clone() * 2;
                if tau1
                {
                    vars.insert(
                        0,
                        (
                            "tau".to_string(),
                            "tau".to_string(),
                            Num(tau.clone().into()),
                        ),
                    );
                }
                if tau2
                {
                    vars.push(('τ'.to_string(), "tau".to_string(), Num(tau.into())))
                }
            }
        }
    }
    if args.contains('e')
    {
        let e = Float::with_val(options.prec, 1).exp();
        vars.push(("e".to_string(), "e".to_string(), Num(e.into())))
    }
    vars.iter()
        .map(|a| {
            if options.small_e
            {
                a.clone()
            }
            else
            {
                (a.0.clone(), a.1.replace('e', "E"), a.2.clone())
            }
        })
        .collect()
}
pub fn get_vars(options: Options) -> Vec<(String, String, NumStr)>
{
    let pi = Float::with_val(options.prec, Pi);
    let tau: Float = pi.clone() * 2;
    let phi: Float = (1 + Float::with_val(options.prec, 5).sqrt()) / 2;
    let e = Float::with_val(options.prec, 1).exp();
    vec![
        (
            "phi".to_string(),
            "phi".to_string(),
            Num(phi.clone().into()),
        ),
        (
            "tau".to_string(),
            "tau".to_string(),
            Num(tau.clone().into()),
        ),
        (
            "ec".to_string(),
            "1.602176634e-19".to_string(),
            Num(Complex::parse("1.602176634e-19")
                .unwrap()
                .complete((options.prec, options.prec))),
        ),
        (
            "kB".to_string(),
            "1.380649e-23".to_string(),
            Num(Complex::parse("1.380649e-23")
                .unwrap()
                .complete((options.prec, options.prec))),
        ),
        (
            "me".to_string(),
            "9.1093837015e-31".to_string(),
            Num(Complex::parse("9.1093837015e-31")
                .unwrap()
                .complete((options.prec, options.prec))),
        ),
        (
            "mn".to_string(),
            "1.67492749804e-27".to_string(),
            Num(Complex::parse("1.67492749804e-27")
                .unwrap()
                .complete((options.prec, options.prec))),
        ),
        (
            "mp".to_string(),
            "1.67262192369e-27".to_string(),
            Num(Complex::parse("1.67262192369e-27")
                .unwrap()
                .complete((options.prec, options.prec))),
        ),
        (
            "Na".to_string(),
            "6.02214076e23".to_string(),
            Num(Complex::parse("6.02214076e23")
                .unwrap()
                .complete((options.prec, options.prec))),
        ),
        ("pi".to_string(), "pi".to_string(), Num(pi.clone().into())),
        (
            "c".to_string(),
            "299792458".to_string(),
            Num(Complex::parse("299792458")
                .unwrap()
                .complete((options.prec, options.prec))),
        ),
        ("e".to_string(), "e".to_string(), Num(e.into())),
        (
            "G".to_string(),
            "6.67430e-11".to_string(),
            Num(Complex::parse("6.67430e-11")
                .unwrap()
                .complete((options.prec, options.prec))),
        ),
        (
            "g".to_string(),
            "9.80665".to_string(),
            Num(Complex::parse("9.80665")
                .unwrap()
                .complete((options.prec, options.prec))),
        ),
        (
            "h".to_string(),
            "6.62607015e-34".to_string(),
            Num(Complex::parse("6.62607015e-34")
                .unwrap()
                .complete((options.prec, options.prec))),
        ),
        (
            "k".to_string(),
            "8.9875517923e9".to_string(),
            Num(Complex::parse("8.9875517923e9")
                .unwrap()
                .complete((options.prec, options.prec))),
        ),
        (
            "R".to_string(),
            "8.31446261815324".to_string(),
            Num(Complex::parse("8.31446261815324")
                .unwrap()
                .complete((options.prec, options.prec))),
        ),
        ("φ".to_string(), "phi".to_string(), Num(phi.into())),
        ("π".to_string(), "pi".to_string(), Num(pi.into())),
        ("τ".to_string(), "tau".to_string(), Num(tau.into())),
    ]
    .iter()
    .map(|a| {
        if options.small_e
        {
            a.clone()
        }
        else
        {
            (a.0.clone(), a.1.replace('e', "E"), a.2.clone())
        }
    })
    .collect()
}