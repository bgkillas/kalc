// as per continued fraction expansion
use crate::Options;
use rug::{float::Constant::Pi, ops::Pow, Float, Integer};
pub fn fraction(value: Float, options: Options) -> String
{
    if value.clone().fract().is_zero() || !value.is_finite()
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
                    "{}{}({})",
                    sign,
                    if i == 1 { "sqrt" } else { "cbrt" },
                    orig.to_integer().unwrap()
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
        for n in 0..32
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
                    format!(
                        "{sign}{}({}{}",
                        if i == 1 { "sqrt" } else { "cbrt" },
                        last,
                        if recip == 1
                        {
                            ")".to_string()
                        }
                        else if i == 1
                        {
                            let (root, rem) = recip.clone().sqrt_rem(Integer::new());
                            if rem == 0
                            {
                                ")/".to_owned() + &root.to_string()
                            }
                            else
                            {
                                "/".to_owned() + &recip.to_string() + ")"
                            }
                        }
                        else
                        {
                            let (root, rem) = recip.clone().root_rem(Integer::new(), 3);
                            if rem == 0
                            {
                                ")/".to_owned() + &root.to_string()
                            }
                            else
                            {
                                "/".to_owned() + &recip.to_string() + ")"
                            }
                        }
                    )
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
                            else
                            {
                                "/".to_owned() + "(" + &recip.to_string() + "e" + ")"
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
