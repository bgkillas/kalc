// as per continued fraction expansion
use crate::Options;
use rug::{float::Constant::Pi, Float};
pub fn fraction(value: Float, options: Options) -> String
{
    if value.clone().fract().is_zero()
    {
        return String::new();
    }
    let rt3 = Float::with_val(options.prec, 3).sqrt();
    let values = [
        Float::with_val(options.prec, 1.0),
        if options.tau
        {
            2 * Float::with_val(options.prec, Pi)
        }
        else
        {
            Float::with_val(options.prec, Pi)
        },
        Float::with_val(options.prec, 2).sqrt(),
        rt3.clone(),
        Float::with_val(options.prec, 2 + rt3.clone()).sqrt(),
        Float::with_val(options.prec, 2 - rt3).sqrt(),
    ];
    let sign: String = if value < 0.0
    {
        "-".to_string()
    }
    else
    {
        "".to_string()
    };
    let val = value.abs();
    let tau = if options.tau { "τ" } else { "π" };
    for (i, constant) in values.iter().enumerate()
    {
        let orig = val.clone() / constant;
        if orig.clone().fract().is_zero()
        {
            return if i == 0
            {
                String::new()
            }
            else
            {
                format!(
                    "{}{}{}",
                    sign,
                    if orig == 1.0
                    {
                        "".to_string()
                    }
                    else
                    {
                        orig.to_integer().unwrap().to_string()
                    },
                    match i
                    {
                        1 => tau,
                        2 => "sqrt(2)",
                        3 => "sqrt(3)",
                        4 => "sqrt(2+sqrt(3))",
                        5 => "sqrt(2-sqrt(3))",
                        _ => "",
                    }
                )
            };
        }
        let mut number = orig.clone().fract();
        let mut nums = Vec::new();
        for _ in 0..=options.frac_iter
        {
            let mut recip = number.clone().recip();
            let fract = recip.clone().fract();
            if fract < 1e-36
            {
                let mut last = Float::with_val(options.prec, 1);
                for j in (0..nums.len()).rev()
                {
                    last = recip.clone();
                    recip *= &nums[j];
                }
                let recip = recip.to_integer().unwrap();
                let last = (last + recip.clone() * orig.trunc()).to_integer().unwrap();
                return if (recip == 1 && i == 0)
                    || recip.to_string().len() > options.decimal_places
                    || last.to_string().len() > options.decimal_places
                {
                    String::new()
                }
                else
                {
                    format!(
                        "{sign}{}{}{}",
                        if last == 1 && i != 0
                        {
                            "".to_string()
                        }
                        else
                        {
                            last.to_string()
                        },
                        match i
                        {
                            0 => "",
                            1 => tau,
                            2 => "sqrt(2)",
                            3 => "sqrt(3)",
                            4 => "sqrt(2+sqrt(3))",
                            5 => "sqrt(2-sqrt(3))",
                            _ => "",
                        },
                        if recip == 1
                        {
                            "".to_string()
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