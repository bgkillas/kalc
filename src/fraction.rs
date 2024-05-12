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
        if orig.clone().fract() < 1e-32
        {
            return if i == 0
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
                        orig.to_integer().unwrap().to_string()
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
        let mut nums = Vec::new();
        for _ in 0..32
        {
            let mut recip = number.clone().recip();
            let fract = recip.clone().fract();
            if fract < 1e-32
            {
                let mut last = Float::with_val(options.prec, 1);
                for j in (0..nums.len()).rev()
                {
                    last.clone_from(&recip);
                    recip *= &nums[j];
                }
                let recip = match recip.to_integer()
                {
                    Some(n) => n,
                    None => return String::new(),
                };
                let last = (last + recip.clone() * orig.trunc()).to_integer().unwrap();
                return if (recip == 1 && i == 0)
                    || recip.to_string().len() > options.decimal_places
                    || last.to_string().len() > options.decimal_places
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
                            0 => String::new(),
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
            nums.push(recip);
            number = fract;
        }
    }
    String::new()
}
