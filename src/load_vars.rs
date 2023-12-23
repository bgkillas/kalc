use crate::{
    complex::{NumStr, NumStr::Num},
    Options,
};
use rug::{float::Constant::Pi, ops::CompleteRound, Complex, Float};
pub fn get_cli_vars(options: Options, args: &[String])
    -> Vec<(String, Vec<NumStr>, NumStr, String)>
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
            "kB".to_string(),
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
            "me".to_string(),
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
            "mn".to_string(),
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
            "mp".to_string(),
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
            "Na".to_string(),
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
            "c".to_string(),
            Vec::new(),
            Num(Complex::parse("299792458").unwrap().complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains('G')
    {
        vars.push((
            "G".to_string(),
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
            "g".to_string(),
            Vec::new(),
            Num(Complex::parse("9.80665").unwrap().complete(options.prec)),
            "".to_string(),
        ));
    }
    if args.contains('h')
    {
        vars.push((
            "h".to_string(),
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
            "k".to_string(),
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
            "R".to_string(),
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
                        "phi".to_string(),
                        Vec::new(),
                        Num(phi.clone().into()),
                        "".to_string(),
                    ),
                )
            }
            if phi2
            {
                vars.push(("φ".to_string(), Vec::new(), Num(phi.into()), "".to_string()))
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
                        "pi".to_string(),
                        Vec::new(),
                        Num(pi.clone().into()),
                        "".to_string(),
                    ),
                );
            }
            if pi2
            {
                vars.push((
                    'π'.to_string(),
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
                            "tau".to_string(),
                            Vec::new(),
                            Num(tau.clone().into()),
                            "".to_string(),
                        ),
                    );
                }
                if tau2
                {
                    vars.push(('τ'.to_string(), Vec::new(), Num(tau.into()), "".to_string()))
                }
            }
        }
    }
    if args.contains('e')
    {
        let e = Float::with_val(options.prec.0, 1).exp();
        vars.push(("e".to_string(), Vec::new(), Num(e.into()), "".to_string()))
    }
    vars
}
pub fn get_vars(options: Options) -> Vec<(String, Vec<NumStr>, NumStr, String)>
{
    let pi = Float::with_val(options.prec.0, Pi);
    let tau: Float = pi.clone() * 2;
    let phi: Float = (1 + Float::with_val(options.prec.0, 5).sqrt()) / 2;
    let e = Float::with_val(options.prec.0, 1).exp();
    vec![
        (
            "phi".to_string(),
            Vec::new(),
            Num(phi.clone().into()),
            "".to_string(),
        ),
        (
            "tau".to_string(),
            Vec::new(),
            Num(tau.clone().into()),
            "".to_string(),
        ),
        (
            "ec".to_string(),
            Vec::new(),
            Num(Complex::parse("1.602176634e-19")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            "kB".to_string(),
            Vec::new(),
            Num(Complex::parse("1.380649e-23")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            "me".to_string(),
            Vec::new(),
            Num(Complex::parse("9.1093837015e-31")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            "mn".to_string(),
            Vec::new(),
            Num(Complex::parse("1.67492749804e-27")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            "mp".to_string(),
            Vec::new(),
            Num(Complex::parse("1.67262192369e-27")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            "Na".to_string(),
            Vec::new(),
            Num(Complex::parse("6.02214076e23")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            "pi".to_string(),
            Vec::new(),
            Num(pi.clone().into()),
            "".to_string(),
        ),
        (
            "c".to_string(),
            Vec::new(),
            Num(Complex::parse("299792458").unwrap().complete(options.prec)),
            "".to_string(),
        ),
        ("e".to_string(), Vec::new(), Num(e.into()), "".to_string()),
        (
            "G".to_string(),
            Vec::new(),
            Num(Complex::parse("6.67430e-11")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            "g".to_string(),
            Vec::new(),
            Num(Complex::parse("9.80665").unwrap().complete(options.prec)),
            "".to_string(),
        ),
        (
            "h".to_string(),
            Vec::new(),
            Num(Complex::parse("6.62607015e-34")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            "k".to_string(),
            Vec::new(),
            Num(Complex::parse("8.9875517923e9")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        (
            "R".to_string(),
            Vec::new(),
            Num(Complex::parse("8.31446261815324")
                .unwrap()
                .complete(options.prec)),
            "".to_string(),
        ),
        ("φ".to_string(), Vec::new(), Num(phi.into()), "".to_string()),
        ("π".to_string(), Vec::new(), Num(pi.into()), "".to_string()),
        ("τ".to_string(), Vec::new(), Num(tau.into()), "".to_string()),
    ]
}