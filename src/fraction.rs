// as per continued fraction expansion
use std::f64::consts::{E, PI};
pub fn fraction(value:f64) -> String
{
    let mut nums:Vec<f64> = vec![];
    let values = [1.0, PI, 2f64.sqrt(), 3f64.sqrt(), 5f64.sqrt(), E];
    let sign:String = if value < 0.0 { "-".to_string() } else { "".to_string() };
    let value = value.abs();
    let (mut number, mut recip, mut fract, mut orig);
    for (i, constant) in values.iter().enumerate()
    {
        orig = value / constant;
        if orig.fract() == 0.0
        {
            return if i == 0
            {
                "".to_string()
            }
            else
            {
                (if orig == 1.0 { "".to_string() } else { orig.to_string() }
                 + match i
                 {
                     1 => "π",
                     2 => "sqrt(2)",
                     3 => "sqrt(3)",
                     4 => "sqrt(5)",
                     5 => "e",
                     _ => "",
                 })
            };
        }
        number = orig.fract();
        nums.clear();
        for _ in 0..=20
        {
            recip = number.recip();
            fract = recip.fract();
            if fract < 1e-6
            {
                let mut last = 1.0;
                for j in (0..nums.len()).rev()
                {
                    last = recip;
                    recip *= nums[j];
                }
                recip = recip.round();
                last = (last + recip * orig.trunc()).round();
                return format!("{sign}{}{}{}",
                               if last == 1.0 && i != 0 { "".to_string() } else { last.to_string() },
                               match i
                               {
                                   0 => "",
                                   1 => "π",
                                   2 => "sqrt(2)",
                                   3 => "sqrt(3)",
                                   4 => "sqrt(5)",
                                   5 => "e",
                                   _ => "",
                               },
                               if recip == 1.0 { "".to_string() } else { "/".to_owned() + &recip.to_string() });
            }
            nums.push(recip);
            number = fract;
        }
    }
    String::new()
}