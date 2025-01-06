// as per continued fraction expansion
use crate::{complex::prime_factors, Colors, Number, Options};
use rug::{float::Constant::Pi, ops::Pow, Complex, Float, Integer};
pub fn fraction(value: Float, options: Options, colors: &Colors, n: usize) -> String
{
    if value.clone().fract().is_zero() || !value.is_finite() || options.prec < 128
    {
        return String::new();
    }
    let e = Float::with_val(options.prec, 1.0).exp();
    let pi = Float::with_val(options.prec, Pi);
    let sign: String = if value.is_sign_negative()
    {
        '-'.to_string()
    }
    else
    {
        String::new()
    };
    let val = value.abs();
    for i in 0..=5
    {
        let orig = match i
        {
            0 => val.clone(),
            1 => val.clone().pow(2),
            2 => val.clone().pow(3),
            3 => val.clone() / pi.clone(),
            4 => val.clone() / e.clone(),
            5 => val.clone() * e.clone(),
            _ => break,
        };
        if ((i == 1 || i == 2) && orig.clone().fract() < 1e-32) || orig.clone().fract().is_zero()
        {
            return if i == 0 || ((i == 1 || i == 2) && orig < 1e-32)
            {
                String::new()
            }
            else if i == 1 || i == 2
            {
                if orig == 1
                {
                    String::new()
                }
                else
                {
                    let mut num = orig.to_integer().unwrap();
                    let mut mul = String::new();
                    if num <= 65536
                    {
                        let pf = prime_factors(num.clone());
                        let mut n = Integer::from(1);
                        for p in pf
                        {
                            n *= p.0.clone().pow((p.1 / if i == 1 { 2 } else { 3 }) as u32);
                            num /= p.0.pow((p.1 - (p.1 % if i == 1 { 2 } else { 3 })) as u32)
                        }
                        if n != 1
                        {
                            mul = n.to_string()
                        }
                    }
                    format!(
                        "{}{}{}{}{}{}",
                        sign,
                        mul,
                        if i == 1 { "sqrt" } else { "cbrt" },
                        if options.color == crate::Auto::True
                        {
                            colors.brackets[n % colors.brackets.len()].to_owned()
                                + "("
                                + &colors.text
                        }
                        else
                        {
                            "(".to_string()
                        },
                        num,
                        if options.color == crate::Auto::True
                        {
                            colors.brackets[n % colors.brackets.len()].to_owned()
                                + ")"
                                + &colors.text
                        }
                        else
                        {
                            ")".to_string()
                        }
                    )
                }
            }
            else
            {
                format!(
                    "{}{}{}",
                    sign,
                    if orig == 1.0 && i != 5
                    {
                        String::new()
                    }
                    else
                    {
                        orig.to_integer().unwrap_or_default().to_string()
                    },
                    match i
                    {
                        3 => "π",
                        4 => "e",
                        5 => "/e",
                        _ => "",
                    }
                )
            };
        }
        let mut number = orig.clone().fract();
        let mut mult = Float::with_val(options.prec, 1);
        let mut first: Float = Float::new(options.prec);
        for j in 0..64
        {
            let mut recip = number.clone().recip();
            let fract = recip.clone().fract();
            if fract > 1e-32
            {
                if j == 0
                {
                    first.clone_from(&recip);
                }
                mult *= recip;
                number = fract;
            }
            else
            {
                recip *= mult.clone();
                let last = recip.clone() / if j == 0 { &recip } else { &first };
                let mut recip = match recip.to_integer()
                {
                    Some(n) => n,
                    None => return String::new(),
                };
                let mut last = (last + recip.clone() * orig.trunc())
                    .to_integer()
                    .unwrap_or_default();
                return if (recip == 1 && i == 0)
                    || recip.to_string().len() > options.decimal_places
                    || last.to_string().len() > options.decimal_places
                    || last == 0
                {
                    String::new()
                }
                else if i == 1 || i == 2
                {
                    let lb = if options.color == crate::Auto::True
                    {
                        colors.brackets[n % colors.brackets.len()].to_owned() + "(" + &colors.text
                    }
                    else
                    {
                        "(".to_string()
                    };
                    let rb = if options.color == crate::Auto::True
                    {
                        colors.brackets[n % colors.brackets.len()].to_owned() + ")" + &colors.text
                    }
                    else
                    {
                        ")".to_string()
                    };
                    if recip == 1
                    {
                        let mut mul = String::new();
                        if last <= 65536
                        {
                            let pf = prime_factors(last.clone());
                            let mut n = Integer::from(1);
                            for p in pf
                            {
                                n *= p.0.clone().pow((p.1 / if i == 1 { 2 } else { 3 }) as u32);
                                last /= p.0.pow((p.1 - (p.1 % if i == 1 { 2 } else { 3 })) as u32)
                            }
                            if n != 1
                            {
                                mul = n.to_string()
                            }
                        }
                        sign.to_owned()
                            + &mul
                            + if i == 1 { "sqrt" } else { "cbrt" }
                            + &lb
                            + &last.to_string()
                            + &rb
                    }
                    else
                    {
                        let mut mul = String::new();
                        if last <= 65536
                        {
                            let pf = prime_factors(last.clone());
                            let mut n = Integer::from(1);
                            for p in pf
                            {
                                n *= p.0.clone().pow((p.1 / if i == 1 { 2 } else { 3 }) as u32);
                                last /= p.0.pow((p.1 - (p.1 % if i == 1 { 2 } else { 3 })) as u32)
                            }
                            if n != 1
                            {
                                mul = n.to_string()
                            }
                        }
                        let mut div = Integer::from(1);
                        if recip <= 65536
                        {
                            let pf = prime_factors(recip.clone());
                            let mut n = Integer::from(1);
                            for p in pf
                            {
                                n *= p.0.clone().pow((p.1 / if i == 1 { 2 } else { 3 }) as u32);
                                recip /= p.0.pow((p.1 - (p.1 % if i == 1 { 2 } else { 3 })) as u32)
                            }
                            if n != 1
                            {
                                div = n
                            }
                        }
                        match (last == 1, recip == 1)
                        {
                            (false, false) =>
                            {
                                div *= recip.clone();
                                format!(
                                    "{sign}{mul}{}{}{}{}{}",
                                    if i == 1 { "sqrt" } else { "cbrt" },
                                    lb,
                                    last * if i == 1 { recip } else { recip.pow(2) },
                                    rb,
                                    if div != 1
                                    {
                                        "/".to_owned() + &div.to_string()
                                    }
                                    else
                                    {
                                        String::new()
                                    }
                                )
                            }
                            (false, true) =>
                            {
                                format!(
                                    "{sign}{mul}{}{}{}{}{}",
                                    if i == 1 { "sqrt" } else { "cbrt" },
                                    lb,
                                    last,
                                    rb,
                                    if div != 1
                                    {
                                        "/".to_owned() + &div.to_string()
                                    }
                                    else
                                    {
                                        String::new()
                                    }
                                )
                            }
                            (true, false) =>
                            {
                                div *= recip.clone();
                                format!(
                                    "{sign}{}/{}{}{}{}",
                                    if mul.is_empty() { "1" } else { &mul },
                                    if i == 1 { "sqrt" } else { "cbrt" },
                                    lb,
                                    div,
                                    rb
                                )
                            }
                            _ => String::new(),
                        }
                    }
                }
                else
                {
                    format!(
                        "{sign}{}{}{}",
                        if (last == 1 && i != 0) || i == 5
                        {
                            String::new()
                        }
                        else
                        {
                            last.to_string()
                        },
                        match i
                        {
                            3 => "π".to_string(),
                            4 => "e".to_string(),
                            5 => last.to_string(),
                            _ => String::new(),
                        },
                        if i == 5
                        {
                            if recip == 1
                            {
                                "/".to_owned() + "e"
                            }
                            else if options.color == crate::Auto::True
                            {
                                format!(
                                    "/{}({}{}e{}){}",
                                    colors.brackets[n % colors.brackets.len()],
                                    colors.text,
                                    recip,
                                    colors.brackets[n % colors.brackets.len()],
                                    colors.text
                                )
                            }
                            else
                            {
                                format!("/({}e)", recip)
                            }
                        }
                        else if recip == 1
                        {
                            String::new()
                        }
                        else
                        {
                            "/".to_owned() + &recip.to_string()
                        }
                    )
                };
            }
        }
    }
    String::new()
}
pub fn rationalize(value: Float, options: Options) -> Option<(Integer, Integer)>
{
    if !value.is_finite() || value.is_zero()
    {
        return None;
    }
    if value.clone().fract().is_zero()
    {
        return Some((value.to_integer().unwrap_or_default(), Integer::from(1)));
    }
    let mut number = value.clone().fract();
    let mut mult = Float::with_val(options.prec, 1);
    let mut first: Float = Float::new(options.prec);
    for j in 0..256
    {
        let mut recip = number.clone().recip();
        let fract = recip.clone().fract();
        if fract > 1e-32
        {
            if j == 0
            {
                first.clone_from(&recip);
            }
            mult *= recip;
            number = fract;
        }
        else
        {
            recip *= mult.clone();
            let last = recip.clone() / if j == 0 { &recip } else { &first };
            let recip = recip.to_integer()?;
            let last = (last + recip.clone() * value.trunc())
                .to_integer()
                .unwrap_or_default();
            return if recip == 1 || last == 0
            {
                None
            }
            else
            {
                Some((last, recip))
            };
        }
    }
    None
}
pub fn c_to_rational(value: Complex, options: Options) -> Vec<Number>
{
    let sign_re = value.real().is_sign_positive();
    let sign_im = value.real().is_sign_positive();
    let re = rationalize(value.real().clone().abs(), options);
    let im = rationalize(value.imag().clone().abs(), options);
    let mut vec = Vec::new();
    if let Some(n) = re
    {
        vec.push(Number::from(
            Complex::with_val(options.prec, if sign_re { n.0 } else { -n.0 }),
            None,
        ));
        vec.push(Number::from(Complex::with_val(options.prec, n.1), None));
    }
    else
    {
        vec.push(Number::from(Complex::new(options.prec), None));
        vec.push(Number::from(Complex::with_val(options.prec, 1), None));
    }
    if let Some(n) = im
    {
        vec.push(Number::from(
            Complex::with_val(options.prec, if sign_im { n.0 } else { -n.0 }),
            None,
        ));
        vec.push(Number::from(Complex::with_val(options.prec, n.1), None));
    }
    vec
}