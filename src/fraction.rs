// as per continued fraction expansion
use crate::{Colors, Options};
use rug::{float::Constant::Pi, ops::Pow, Float, Integer};
pub fn fraction(value: Float, options: Options, colors: &Colors, n: usize) -> String
{
    if value.clone().fract().is_zero() || !value.is_finite() || options.prec < 128
    {
        return String::new();
    }
    let e = Float::with_val(options.prec, 1.0).exp();
    let pi = Float::with_val(options.prec, Pi);
    let sign: String = if value < 0.0
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
                format!(
                    "{}{}{}{}{}",
                    sign,
                    if i == 1 { "sqrt" } else { "cbrt" },
                    if options.color == crate::Auto::True
                    {
                        colors.brackets[n % colors.brackets.len()].to_owned() + "(" + &colors.text
                    }
                    else
                    {
                        "(".to_string()
                    },
                    orig.to_integer().unwrap(),
                    if options.color == crate::Auto::True
                    {
                        colors.brackets[n % colors.brackets.len()].to_owned() + ")" + &colors.text
                    }
                    else
                    {
                        ")".to_string()
                    }
                )
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
        for n in 0..64
        {
            let mut recip = number.clone().recip();
            let fract = recip.clone().fract();
            if fract > 1e-32
            {
                if n == 0
                {
                    first.clone_from(&recip);
                }
                mult *= recip;
                number = fract;
            }
            else
            {
                recip *= mult.clone();
                let last = recip.clone() / if n == 0 { &recip } else { &first };
                let recip = match recip.to_integer()
                {
                    Some(n) => n,
                    None => return String::new(),
                };
                let last = (last + recip.clone() * orig.trunc())
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
                        (if i == 1 { "sqrt" } else { "cbrt" }).to_owned()
                            + &lb
                            + &last.to_string()
                            + &rb
                    }
                    else
                    {
                        let ((num_root, num_rem), (den_root, den_rem)) = if i == 1
                        {
                            (
                                last.clone().sqrt_rem(Integer::new()),
                                recip.clone().sqrt_rem(Integer::new()),
                            )
                        }
                        else
                        {
                            (
                                last.clone().root_rem(Integer::new(), 3),
                                recip.clone().root_rem(Integer::new(), 3),
                            )
                        };
                        match (num_rem == 0, den_rem == 0)
                        {
                            (false, false) =>
                            {
                                format!(
                                    "{sign}{}{}{}/{}{}",
                                    if i == 1 { "sqrt" } else { "cbrt" },
                                    lb,
                                    last,
                                    recip,
                                    rb
                                )
                            }
                            (false, true) =>
                            {
                                format!(
                                    "{sign}{}{}{}{}/{}",
                                    if i == 1 { "sqrt" } else { "cbrt" },
                                    lb,
                                    last,
                                    rb,
                                    den_root
                                )
                            }
                            (true, false) =>
                            {
                                format!(
                                    "{sign}{}/{}{}{}{}",
                                    num_root,
                                    if i == 1 { "sqrt" } else { "cbrt" },
                                    lb,
                                    recip,
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
