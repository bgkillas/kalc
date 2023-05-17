// as per continued fraction expansion
use std::f64::consts::PI;
pub fn fraction(value:f64) -> String
{
    let eps = 1e-6;
    let mut nums:Vec<f64> = vec![];
    let values = [1.0, PI, 2f64.sqrt(), 3f64.sqrt()];
    let sign:String = if value < 0.0 { "-".to_string() } else { "".to_string() };
    let value = value.abs();
    let mut number;
    let mut recip;
    let mut fract;
    let mut orig;
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
                     _ => "",
                 })
            };
        }
        number = orig.fract();
        nums.clear();
        for _ in 0..=10
        {
            recip = number.recip();
            fract = recip.fract();
            if fract < eps
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